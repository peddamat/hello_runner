use dll_syringe::{Syringe, process::OwnedProcess};
use std::{thread, time, env};
use windows::Win32::UI::WindowsAndMessaging::PostMessageA;
use windows::Win32::Foundation::*;

use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowThreadProcessId, PostThreadMessageA, SetWindowsHookExA, HOOKPROC, WH_CALLWNDPROC, WH_GETMESSAGE, WM_NULL};
use windows::Win32::Foundation::{WPARAM, LPARAM};
use windows::core::PCSTR;
use windows::core::s;

fn main() {
    // eventlog::register("Example Log").unwrap();
    // find target process by name
    // let target_process =
        // OwnedProcess::find_first_by_name("notepad.exe").unwrap();
        // OwnedProcess::find_first_by_name("sample_create_window.exe").unwrap();

    // create a new syringe for the target process
    // let syringe = Syringe::for_process(target_process);

    println!("Current directory: {}", std::env::current_dir().unwrap().display());

    // inject the payload into the target process
    // let injected_payload = syringe.inject("hello_inject\\target\\debug\\deps\\hello.dll").unwrap();
    // let injected_payload = syringe.inject("target\\debug\\deps\\hello.dll").unwrap();
    // let injected_payload = syringe.inject("..\\target\\debug\\deps\\hello.dll").unwrap();
    // println!("DLL injected successfully!");


    let dll_location = "..\\target\\debug\\deps\\hello.dll";
    let dll_path_str = std::ffi::CString::new(dll_location).unwrap();
    let hw = unsafe{ FindWindowA(PCSTR(std::ptr::null()), PCSTR("This is a sample window\0".as_ptr())) };
    // let hw = unsafe{ FindWindowA(PCSTR(std::ptr::null()), PCSTR("Untitled - Notepad\0".as_ptr())) };
    let mut process_id = 0;
    let tid = unsafe { GetWindowThreadProcessId(hw, Some(&mut process_id)) };

    println!("Found window: {}", tid);

    if tid == 0 {
        panic!("{}","Failed to get thread id");
    }


    unsafe {
        let stub_module = match LoadLibraryA(PCSTR(dll_path_str.as_ptr() as *const u8)) {
            Ok(handle) => handle,
            Err(_) => {
                println!("Error in LoadLibraryA: {:?}", GetLastError());
                return
            }
        };
        let stub_callback: HOOKPROC = std::mem::transmute(GetProcAddress(stub_module, PCSTR("CallWndProc\0".as_ptr()))) ;

        // let shitter = GetProcAddress(stub_module, s!("CallWndProc")).unwrap();
        // let dick: unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT = std::mem::transmute(&shitter);
        // let dick: HOOKPROC = std::mem::transmute(shitter);

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

    }

    // do something else
    let ten_millis = time::Duration::from_secs(60);
    println!("Sleeping for 30 secs...");
    thread::sleep(ten_millis);

    // eject the payload from the target (optional)
    // syringe.eject(injected_payload).unwrap();
}