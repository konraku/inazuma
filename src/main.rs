// リリースビルド時のみコンソールを非表示
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod worker;
mod virtual_pad;
mod process;

//  メイン
fn main() -> Result<(), eframe::Error> {
    // ウィンドウ設定
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 215.0])     //ウィンドウサイズ (幅, 高さ)
            .with_min_inner_size([300.0, 250.0]) // 最小サイズ（固定用）
            .with_max_inner_size([300.0, 250.0]) // 最大サイズ（固定用）
            .with_resizable(false),              // リサイズ不可
        ..Default::default()                     // その他の設定はデフォルト値
    };

    // ウィンドウを起動し、描画ループを開始
    // run_native: OSネイティブなウィンドウを開き、アプリを実行するメイン関数
    eframe::run_native(
        // タイトル設定
        "inazuma",
        options,
        //アプリの初期化とインスタンス
        Box::new(|_cc| Ok(Box::<app::A>::default())),
    )
}