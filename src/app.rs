use crate::worker::Worker;
use crate::virtual_pad::VirtualPad;
//use crate::process::{self, ProcessInfo};
use std::sync::{Arc, Mutex};
use eframe::egui;

pub struct A {
    //pub selection_process: String,
    pub selection_button: String,
    pub interval_ms: u64,
    //processes: Vec<ProcessInfo>,
    worker: Option<Worker>,
    is_running: bool,  // 実行状態を追跡
}

// データの初期化
impl Default for A {
    fn default() -> Self {
        // 起動時にプロセス一覧を取得
        //let processes = process::get_processes_with_window();
        
        // デバッグ用にコンソール出力
        //process::print_processes_with_window();

        /*
        let initial_process = processes
            .first()
            .map(|p| format!("{} - {}", p.name, p.window_title))
            .unwrap_or_else(|| "プロセスなし".to_string());
        */

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
            //selection_process: initial_process,
            selection_button: "B".to_string(),
            interval_ms: 50,
            //processes,
            worker,
            is_running: false,
        }
    }
}

impl A {
    /// 停止処理（共通化）
    fn stop(&mut self) {
        if let Some(worker) = &self.worker {
            worker.stop();
        }
        self.is_running = false;
    }
}

impl eframe::App for A {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // タイトルを状態に応じて変更
        let title = if self.is_running {
            "inazuma - 実行中"
        } else {
            "inazuma - 停止"
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.to_string()));

        egui::CentralPanel::default().show(ctx, |ui| {

            // --- プロセス選択 ---
            /*
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
            */
            
            // --- 入力キー選択 ---
            ui.label("Key:");
            let prev_button = self.selection_button.clone();
            egui::ComboBox::from_id_salt("key_combo")
                .selected_text(&self.selection_button)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for key in ["A", "B", "X", "Y", "LB", "RB"] {
                        ui.selectable_value(&mut self.selection_button, key.to_string(), key);
                    }
                });
            // キーが変更されたら停止
            if prev_button != self.selection_button && self.is_running {
                println!("キー変更により停止");
                self.stop();
            }
            ui.add_space(10.0);

            // --- 入力間隔選択 ---
            ui.label("Interval (ms):");
            let prev_interval = self.interval_ms;
            ui.add(egui::DragValue::new(&mut self.interval_ms).range(100..=10000));
            // 間隔が変更されたら停止
            if prev_interval != self.interval_ms && self.is_running {
                println!("間隔変更により停止");
                self.stop();
            }
            ui.add_space(20.0);

            // --- 開始/停止ボタン ---
            let button_height = 40.0;
            let full_width = ui.available_width();

            // 開始ボタン（青）
            let start_btn = egui::Button::new(
                egui::RichText::new("▶ START").size(16.0).color(egui::Color32::WHITE)
            )
            .fill(egui::Color32::from_rgb(0, 120, 255))
            .min_size(egui::vec2(full_width, button_height));

            if ui.add(start_btn).clicked() {
                println!("-------start-------");
                if let Some(worker) = &self.worker {
                    worker.start(self.selection_button.clone(), self.interval_ms);
                    self.is_running = true;
                } else {
                    eprintln!("Workerが初期化されていません");
                }
            }

            ui.add_space(8.0);

            // 停止ボタン（赤）
            let stop_btn = egui::Button::new(
                egui::RichText::new("■ STOP").size(16.0).color(egui::Color32::WHITE)
            )
            .fill(egui::Color32::from_rgb(220, 50, 50))
            .min_size(egui::vec2(full_width, button_height));

            if ui.add(stop_btn).clicked() {
                println!("--------end--------");
                self.stop();
            }
        });
    }
}