use sysinfo::System;
use std::collections::HashSet; // 重複排除のためにHashSetをインポート

pub struct TargetApplication {
    // ユーザーが選択したプロセス名
    pub selection_process: String,
}

impl TargetApplication {
    pub fn new(selection_process: String) -> Self {
        Self {
            selection_process
        }
    }

    /// 実行中のプロセス名を取得し、重複を排除
    pub fn list_running_processes() -> Vec<String> {
        println!("--- 実行中のプロセス一覧 ---");
        
        // システム情報を保持する構造体を初期化
        let mut system = System::new_all();

        // 情報を最新の状態に更新
        system.refresh_all(); 

        // HashSet:重複を許さないコレクション
        let mut unique_process_names: HashSet<String> = HashSet::new();

        for (_pid, process) in system.processes() {
            // プロセス名を取得し、String型に変換
            let name = process.name().to_string_lossy().to_string();
            
            // HHashSetは既にある要素は無視するため、重複が自動的に排除
            unique_process_names.insert(name);
        }
        let mut process_list: Vec<String> = unique_process_names.into_iter().collect();

        // アルファベット順にソート
        process_list.sort();

        for name in &process_list {
            println!("{}", name);
        }
        println!("--------------------------------------");

        process_list
    }
}