use std::collections::HashMap;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, GetWindowTextLengthW, GetWindowThreadProcessId, IsWindowVisible,
};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub id: u32,
    pub name: String,
    pub window_title: String,
}

pub fn get_processes_with_window() -> Vec<ProcessInfo> {
    let mut window_map: HashMap<u32, String> = HashMap::new();

    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_callback),
            LPARAM(&mut window_map as *mut HashMap<u32, String> as isize),
        );
    }

    let mut processes: Vec<ProcessInfo> = window_map
        .into_iter()
        .filter_map(|(pid, title)| {
            let name = get_process_name(pid).unwrap_or_else(|| "Unknown".to_string());
            Some(ProcessInfo {
                id: pid,
                name,
                window_title: title,
            })
        })
        .collect();

    processes.sort_by(|a, b| a.id.cmp(&b.id));
    processes
}

pub fn print_processes_with_window() {
    let processes = get_processes_with_window();

    println!("{:<8} {:<30} {}", "Id", "ProcessName", "MainWindowTitle");
    println!("{}", "-".repeat(80));

    for p in processes {
        println!("{:<8} {:<30} {}", p.id, p.name, p.window_title);
    }
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let map = &mut *(lparam.0 as *mut HashMap<u32, String>);

        if IsWindowVisible(hwnd).as_bool() {
            let len = GetWindowTextLengthW(hwnd);
            if len > 0 {
                let mut buffer: Vec<u16> = vec![0; (len + 1) as usize];
                let actual_len = GetWindowTextW(hwnd, &mut buffer);

                if actual_len > 0 {
                    let title = OsString::from_wide(&buffer[..actual_len as usize])
                        .to_string_lossy()
                        .to_string();

                    if !title.is_empty() {
                        let mut pid: u32 = 0;
                        GetWindowThreadProcessId(hwnd, Some(&mut pid));

                        if pid != 0 && !map.contains_key(&pid) {
                            map.insert(pid, title);
                        }
                    }
                }
            }
        }
        BOOL(1)
    }
}

fn get_process_name(pid: u32) -> Option<String> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid).ok()?;
        let mut buffer: Vec<u16> = vec![0; 260];
        let len = GetModuleBaseNameW(handle, None, &mut buffer);

        if len > 0 {
            Some(
                OsString::from_wide(&buffer[..len as usize])
                    .to_string_lossy()
                    .to_string(),
            )
        } else {
            None
        }
    }
}