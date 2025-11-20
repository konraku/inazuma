mod app;
mod worker;
mod virtual_pad;

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
        Box::new(|_cc| Ok(Box::<app::A>::default())), //アプリの初期化とインスタンス
    )
}