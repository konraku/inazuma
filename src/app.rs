use crate::worker::Worker;
use crate::virtual_pad::VirtualPad;
use crate::process;
use std::sync::{Arc, Mutex};
use eframe::egui;

// アプリケーションの状態を保持する構造体
pub struct A {
    pub selection_process: String,
    pub selection_button: String,
    pub interval_ms: u64,
    
    // Workerをアプリの状態として持ち続ける
    worker: Option<Worker>, 
}

// データの初期化
impl Default for A {
    fn default() -> Self {

        process::print_processes_with_window();

        // アプリ起動時に1回だけコントローラーとWorkerを作る
        // match:OK/NG
        let worker = match VirtualPad::new() {
            Ok(pad) => {
                // Arc   :Atomically Reference Countedの略。Reとは異なり、複数のスレッド間でデータを安全に共有
                // Mutex :読み取り書き込み問わず排他的に処理
                // Some  :値が存在することを表す
                let pad = Arc::new(Mutex::new(pad));
                Some(Worker::new(pad))
            },
            Err(e) => {
                eprintln!("VirtualPadの接続に失敗しました: {}", e);
                // None:null相当？
                None
            }
        };

        // 戻り値
        Self {
            selection_process: "pleace selection".to_string(),
            selection_button: "B".to_string(),
            interval_ms: 50,
            worker, // 作成したワーカーを保持
        }
    }
}

//  アプリの状態を定義
impl eframe::App for A {
    //  GUIアプリケーションの描画とイベント処理を行うメインループ関数
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // ウィンドウの中央画面
        egui::CentralPanel::default().show(ctx, |ui| {

            // --- プロセス選択 ---
            ui.label("process:");
            egui::ComboBox::from_id_salt("process_combo")
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for key in ["ProcessA", "ProcessB", "ProcessC"] {
                        ui.selectable_value(&mut self.selection_process, key.to_string(), key);
                    }
                });
            ui.add_space(10.0);

            // --- 入力キー選択 ---
            ui.label("Key:");
            egui::ComboBox::from_id_salt("key_combo")
                .selected_text(&self.selection_button)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    // ボタンリスト
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
                // 左側(開始ボタン)
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let mut blue = ui.visuals().clone();
                    blue.widgets.inactive.bg_fill = egui::Color32::from_rgb(0, 100, 255);
                    ui.visuals_mut().widgets = blue.widgets;

                    if ui.button("start").clicked() {
                        println!("-------start-------");
                        // ボタンが押された時だけ Worker を開始する
                        if let Some(worker) = &self.worker {
                            worker.start(self.selection_button.clone(), self.interval_ms);
                        } else {
                            eprintln!("Workerが初期化されていません");
                        }
                    }
                });

                // 右側(停止ボタン)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 右側（停止ボタン）
                    let mut red = ui.visuals().clone();
                    red.widgets.inactive.bg_fill = egui::Color32::from_rgb(200, 0, 0);
                    ui.visuals_mut().widgets = red.widgets;

                    if ui.button("end").clicked() {
                        println!("--------end--------");
                        // ボタンが押された時だけ Worker を停止する
                        if let Some(worker) = &self.worker {
                            worker.stop();
                        }
                    }
                });
            });
        });
    }
}