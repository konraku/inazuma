use crate::worker::Worker;
use crate::virtual_pad::VirtualPad;
use crate::process::{self, ProcessInfo};
use std::sync::{Arc, Mutex};
use eframe::egui;

// アプリケーションの状態を保持する構造体
pub struct A {
    pub selection_process: String,
    pub selection_button: String,
    pub interval_ms: u64,
    
    // プロセス一覧を保持
    processes: Vec<ProcessInfo>,
    
    // Workerをアプリの状態として持ち続ける
    worker: Option<Worker>, 
}

// データの初期化
impl Default for A {
    fn default() -> Self {
        // 起動時にプロセス一覧を取得
        let processes = process::get_processes_with_window();
        
        // デバッグ用にコンソール出力
        process::print_processes_with_window();

        // 初期選択値（プロセスがあれば最初のものを選択）
        let initial_process = processes
            .first()
            .map(|p| format!("{} - {}", p.name, p.window_title))
            .unwrap_or_else(|| "プロセスなし".to_string());

        let worker = match VirtualPad::new() {
            Ok(pad) => {
                let pad = Arc::new(Mutex::new(pad));
                Some(Worker::new(pad))
            },
            Err(e) => {
                eprintln!("VirtualPadの接続に失敗しました: {}", e);
                None
            }
        };

        Self {
            selection_process: initial_process,
            selection_button: "B".to_string(),
            interval_ms: 50,
            processes,
            worker,
        }
    }
}

impl eframe::App for A {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // --- プロセス選択 ---
            ui.horizontal(|ui| {
                ui.label("process:");
                
                // 更新ボタン（プロセス一覧を再取得）
                if ui.button("🔄").clicked() {
                    self.processes = process::get_processes_with_window();
                }
            });
            
            egui::ComboBox::from_id_salt("process_combo")
                .selected_text(&self.selection_process)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    // 取得したプロセス一覧を表示
                    for p in &self.processes {
                        let display = format!("{} - {}", p.name, p.window_title);
                        ui.selectable_value(
                            &mut self.selection_process,
                            display.clone(),
                            display,
                        );
                    }
                });
            ui.add_space(10.0);

            // --- 入力キー選択 ---
            ui.label("Key:");
            egui::ComboBox::from_id_salt("key_combo")
                .selected_text(&self.selection_button)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for key in ["A", "B", "X", "Y", "LB", "RB"] {
                        ui.selectable_value(&mut self.selection_button, key.to_string(), key);
                    }
                });
            ui.add_space(10.0);

            // --- 入力間隔選択 ---
            ui.label("Interval (ms):");
            ui.add(egui::DragValue::new(&mut self.interval_ms).range(100..=10000));
            ui.add_space(20.0);

            // --- 開始/停止ボタン ---
            ui.horizontal(|ui| {

                //開始
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let mut blue = ui.visuals().clone();
                    blue.widgets.inactive.bg_fill = egui::Color32::from_rgb(0, 100, 255);
                    ui.visuals_mut().widgets = blue.widgets;

                    if ui.button("start").clicked() {
                        println!("-------start-------");
                        if let Some(worker) = &self.worker {
                            worker.start(self.selection_button.clone(), self.interval_ms);
                        } else {
                            eprintln!("Workerが初期化されていません");
                        }
                    }
                });

                //停止
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut red = ui.visuals().clone();
                    red.widgets.inactive.bg_fill = egui::Color32::from_rgb(200, 0, 0);
                    ui.visuals_mut().widgets = red.widgets;

                    if ui.button("end").clicked() {
                        println!("--------end--------");
                        if let Some(worker) = &self.worker {
                            worker.stop();
                        }
                    }
                });
            });
        });
    }
}