use eframe::egui;

// アプリケーションの状態を保持する構造体
struct A;

// データの初期化
impl Default for A { 
    fn default() -> Self { 
        Self {}
    } 
}

//  アプリの状態を定義(状態データ:コントローラー的な役割)
impl eframe::App for A {
    //  GUIアプリケーションの描画とイベント処理を行うメインループ関数
    /*
        $mut    :可変参照,
        Context :UIのレイアウト、描画、入力イベントの管理,
        _frame  :タイトルバー、サイズ、終了ボタンなど
     */
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // CentralPanel: ウィンドウの中央全体に描画
        egui::CentralPanel::default().show(ctx, |_ui| {
            // UI部品
        });
    }
}

//  メイン
fn main() -> Result<(), eframe::Error> {
    // ウィンドウ設定
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 150.0]),   //ウィンドウサイズ (幅, 高さ)
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