use std::{sync::{Arc, Mutex}, thread, time::Duration};
use crate::virtual_pad::VirtualPad;

pub struct Worker {
    pad: Arc<Mutex<VirtualPad>>,
    running: Arc<Mutex<bool>>,
}

impl Worker {
    pub fn new(pad: Arc<Mutex<VirtualPad>>) -> Self {
        Self {
            pad,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self, button: String, interval_ms: u64) {
        let pad = self.pad.clone();
        let running = self.running.clone();

        // 実行フラグをON
        *running.lock().unwrap() = true;
        thread::spawn(move || {
            while *running.lock().unwrap() {
                {
                    if let Ok(mut pad_lock) = pad.lock() {
                        let _ = pad_lock.press_button(&button);
                    }
                }                
                thread::sleep(Duration::from_millis(30));

                {
                    if let Ok(mut pad_lock) = pad.lock() {
                        let _ = pad_lock.release_all();
                    }
                }
                thread::sleep(Duration::from_millis(interval_ms));
            }
        });
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }
}