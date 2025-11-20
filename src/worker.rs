use std::{sync::{Arc, Mutex}, thread, time::Duration};
use crate::virtual_pad::VirtualPad;

pub struct Worker {
    pad: Arc<Mutex<VirtualPad>>, // 仮想パッド
    running: Arc<Mutex<bool>>,   // ループフラグ
}

impl Worker {
    pub fn new(pad: Arc<Mutex<VirtualPad>>) -> Self {
        Self {
            pad,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self, button: String, interval_ms: u64) {
        // Arcをクローンし、padやrunningを安全に使用
        let pad = self.pad.clone();
        let running = self.running.clone();

        // 実行フラグをON
        // unwrap()で取り出してtrueに書き換え
        *running.lock().unwrap() = true;

        // spawn:別スレッド生成
        // move 変数の所有権をスレッド内に移動
        thread::spawn(move || {
            while *running.lock().unwrap() {
                {
                    // ロックを取得し、仮想パッドを操作
                    if let Ok(mut pad_lock) = pad.lock() {
                        let _ = pad_lock.press_button(&button);
                    }
                }             
                // 0.03秒待機   
                thread::sleep(Duration::from_millis(30));

                {
                    if let Ok(mut pad_lock) = pad.lock() {
                        let _ = pad_lock.release_all();
                    }
                }

                // 指定した間隔(ms)だけ休む
                thread::sleep(Duration::from_millis(interval_ms));
            }
        });
    }

    pub fn stop(&self) {
        // unwrap()で取り出してfalseに書き換え
        *self.running.lock().unwrap() = false;
    }
}