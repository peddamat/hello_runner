use std::{thread, time};
use windows::core::{s, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowThreadProcessId, SetWindowsHookExA, UnhookWindowsHookEx, HOOKPROC, WH_CALLWNDPROC, WH_GETMESSAGE};

fn main() {
    // eventlog::register("Example Log").unwrap();

    unsafe {

        // let hw = FindWindowA(None, s!("This is a sample window\0");
        let hw = FindWindowA(None, s!("Untitled - Notepad\0"));

        let mut pid = 0;
        let tid = GetWindowThreadProcessId(hw, Some(&mut pid));

        if tid == 0 {
            panic!("{}","Failed to get thread id");
        }

        let stub_module = match LoadLibraryA(s!("..\\target\\debug\\deps\\hello.dll")) {
            Ok(handle) => handle,
            Err(_) => {
                println!("Error in LoadLibraryA: {:?}", GetLastError());
                return
            }
        };

        let stub_callback: HOOKPROC = std::mem::transmute(GetProcAddress(stub_module, s!("CallWndProc")));

        let hook_wndproc = match SetWindowsHookExA(WH_CALLWNDPROC, stub_callback, stub_module, tid) {
            Ok(handle) => handle,
            Err(_) => {
                println!("Error in CallWndProc SetWindowsHookEx: {:?}", GetLastError());
                return
            }
        };

        let stub_callback2: HOOKPROC = std::mem::transmute(GetProcAddress(stub_module, PCSTR("GetMsgProc\0".as_ptr()))) ;
        let hook_getmsg = match SetWindowsHookExA(WH_GETMESSAGE, stub_callback2, stub_module, tid) {
            Ok(handle) => handle,
            Err(_) => {
                println!("Error in GetMsgProc SetWindowsHookEx: {:?}", GetLastError());
                return
            }
        };

        let ten_millis = time::Duration::from_secs(10);
        println!("Sleeping for 30 secs...");
        thread::sleep(ten_millis);

        UnhookWindowsHookEx(hook_wndproc);
        UnhookWindowsHookEx(hook_getmsg);
    }
}