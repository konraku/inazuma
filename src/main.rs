use eframe::egui;

// アプリケーションの状態を保持する構造体
struct A{
    // セレクトボックスの現在の選択肢を保持するフィールド
    selection1: String,
    selection2: String,
    selection3: String,
}

// データの初期化
impl Default for A {
    fn default() -> Self {
        Self {
            // 初期値を設定
            selection1: "processSelect".to_owned(),
            selection2: "buttonSetting(xbox)".to_owned(),
            selection3: "ms".to_owned(),
        }
    }
}

//  アプリの状態を定義(状態データ:コントローラー的な役割)
impl eframe::App for A {
    //  GUIアプリケーションの描画とイベント処理を行うメインループ関数
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // ウィンドウの中央画面
        egui::CentralPanel::default().show(ctx, |ui| {
            let selectOptions1 = ["A", "B", "C"];

            // --- ボックス1 ---
            ui.label(&self.selection1);
            egui::ComboBox::from_id_source(1)
                // 横幅いっぱい
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for option in selectOptions1.iter() {
                        ui.selectable_value(&mut self.selection2, option.to_string(), *option);
                    }
                });
            ui.add_space(8.0);

            // --- ボックス2 ---
            ui.label(&self.selection2);
            egui::ComboBox::from_id_source(2)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for option in selectOptions1.iter() {
                        ui.selectable_value(&mut self.selection2, option.to_string(), *option);
                    }
                });
            ui.add_space(8.0);

            // --- ボックス3 ---
            ui.label(&self.selection3);
            egui::ComboBox::from_id_source(3)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for option in selectOptions1.iter() {
                        ui.selectable_value(&mut self.selection3, option.to_string(), *option);
                    }
                });
            ui.add_space(50.0);


            // --- 開始/停止ボタン ---
            // 左右に配置するために水平レイアウト
            ui.horizontal(|ui| {
                // 左側拡張スペース
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // 青ボタン（開始）
                    let mut blue = ui.visuals().clone();
                    blue.widgets.inactive.bg_fill = egui::Color32::from_rgb(0, 100, 255);
                    ui.visuals_mut().widgets = blue.widgets;

                    if ui.button("start").clicked() {
                        println!("-------start-------");
                    }
                });

                ui.add_space(ui.available_width()); // 中央スペース

                // 右側に停止ボタン
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 赤ボタン（停止）
                    let mut red = ui.visuals().clone();
                    red.widgets.inactive.bg_fill = egui::Color32::from_rgb(200, 0, 0);
                    ui.visuals_mut().widgets = red.widgets;

                    if ui.button("end").clicked() {
                        println!("--------end--------");
                    }
                });
            });
        });
    }
}

//  メイン
fn main() -> Result<(), eframe::Error> {
    // ウィンドウ設定
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 220.0]),   //ウィンドウサイズ (幅, 高さ)
        ..Default::default()                    // その他の設定はデフォルト値
    };

    // ウィンドウを起動し、描画ループを開始
    // run_native: OSネイティブなウィンドウを開き、アプリを実行するメイン関数
    eframe::run_native(
        "inazuma", // ウィンドウタイトル
        options,            //  ウィンドウ設定
        Box::new(|_cc| Ok(Box::<A>::default())), //アプリの初期化とインスタンス
    )
}