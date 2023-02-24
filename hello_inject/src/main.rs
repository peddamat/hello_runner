use dll_syringe::{Syringe, process::OwnedProcess};
use std::{thread, time, env};
use windows::Win32::UI::WindowsAndMessaging::PostMessageA;
use windows::Win32::Foundation::*;

fn main() {
    // find target process by name
    let target_process =
        // OwnedProcess::find_first_by_name("notepad.exe").unwrap();
        OwnedProcess::find_first_by_name("sample_create_window.exe").unwrap();

    // create a new syringe for the target process
    let syringe = Syringe::for_process(target_process);

    println!("Current directory: {}", std::env::current_dir().unwrap().display());

    // inject the payload into the target process
    // let injected_payload = syringe.inject("hello_inject\\target\\debug\\deps\\hello.dll").unwrap();
    // let injected_payload = syringe.inject("target\\debug\\deps\\hello.dll").unwrap();
    let injected_payload = syringe.inject("..\\target\\debug\\deps\\hello.dll").unwrap();
    println!("DLL injected successfully!");

    unsafe { PostMessageA(HWND(0xffff), 0, WPARAM(0), LPARAM(0)) };
    unsafe { PostMessageA(HWND(0xffff), 0, WPARAM(0), LPARAM(0)) };
    unsafe { PostMessageA(HWND(0xffff), 0, WPARAM(0), LPARAM(0)) };


    // do something else
    let ten_millis = time::Duration::from_secs(60);
    println!("Sleeping for 30 secs...");
    thread::sleep(ten_millis);

    // eject the payload from the target (optional)
    syringe.eject(injected_payload).unwrap();
}