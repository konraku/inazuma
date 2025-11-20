use vigem_client::{Client, Xbox360Wired, XButtons, XGamepad, TargetId};
use anyhow::Result;

pub struct VirtualPad {
    pad: Xbox360Wired<Client>, // 仮想コントローラーの通信・制御
    state: XGamepad,           // どのボタンが押されているか
}

impl VirtualPad {
    pub fn new() -> Result<Self> {
        // ViGEmと通信
        let client = Client::connect()?;
        // オブジェクト作成
        let mut pad = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
        // 仮想USB接続(音がなる)
        pad.plugin()?;

        Ok(Self {
            pad,
            // 何も押されていない状態
            state: XGamepad::default(),
        })
    }

    // ボタン操作
    pub fn press_button(&mut self, button: &str) -> Result<()> {
        match button {
            "A" => self.state.buttons.raw |= XButtons::A,
            "B" => self.state.buttons.raw |= XButtons::B,
            "X" => self.state.buttons.raw |= XButtons::X,
            "Y" => self.state.buttons.raw |= XButtons::Y,
            "LB" => self.state.buttons.raw |= XButtons::LB,
            "RB" => self.state.buttons.raw |= XButtons::RB,
            _ => {}
        }

        // 2. 更新した「状態」をドライバに送信する
        self.pad.update(&self.state)?;
        Ok(())
    }

    // ボタン解除
    pub fn release_all(&mut self) -> Result<()> {
        // 状態をリセット（初期化）
        self.state = XGamepad::default();
        
        // リセットした状態（全離し）を送信
        self.pad.update(&self.state)?;
        Ok(())
    }
}