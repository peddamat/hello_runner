use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };
use windows::{ core::*, Win32::UI::WindowsAndMessaging::MessageBoxA, };
use windows::{ Win32::Graphics::Gdi::*, Win32::UI::WindowsAndMessaging::*, };
use windows::Win32::{UI::Shell::{DefSubclassProc, SetWindowSubclass}};
use windows::Win32::System::Threading::{GetCurrentProcessId, GetCurrentThreadId};
use windows::{ Win32::UI::WindowsAndMessaging::{ EnumWindows, IsWindowVisible, GetWindowThreadProcessId }, };
use std::io::{Error, ErrorKind, Result};
use std::mem::MaybeUninit;
use log::{info, trace};

use windows::Win32::{
    Foundation::{BOOL, HINSTANCE, LPARAM, LRESULT, WPARAM},
    System::{SystemServices::DLL_PROCESS_ATTACH,},
    UI::WindowsAndMessaging::{
        CallNextHookEx, SetWindowsHookExW, HHOOK, MB_OK, MSG, WH_GETMESSAGE, WH_CALLWNDPROC,
        WM_COMMAND,
    },
};
use std::mem::transmute;

static mut HOOK: HHOOK = HHOOK(0);

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => attach(dll_module),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    true
}

fn attach(dll_module: HINSTANCE) {
    eventlog::init("Example Log", log::Level::Trace).unwrap();
    info!("Attach called!");
    unsafe {
        let tid = GetCurrentThreadId();
        let pid = GetCurrentProcessId();
        info!("Thread id: {}", tid);
        info!("Process id: {}", pid);

        let hwnd = find_window_by_pid(pid).unwrap();
        info!("Found window handle");

        // Get thread id
        let mut pid = MaybeUninit::<u32>::zeroed();
        let gui_tid = GetWindowThreadProcessId(hwnd, Some(pid.as_mut_ptr()));
        info!("GUI Thread id: {}", gui_tid);

        // let stub_module = unsafe { LoadLibraryA(PCSTR("hello.dll\0".as_ptr() as *const u8)).unwrap() };
        // let stub_callback: HOOKPROC = unsafe { std::mem::transmute(GetProcAddress(dll_module, PCSTR("callback".as_ptr()))) };
        // HOOK = match SetWindowsHookExW(WH_CALLWNDPROC, stub_callback, None, gui_tid) {

        // Doesn't work when injected
        // Works when Load Library is called
        let shitter = GetProcAddress(dll_module, s!("callback")).unwrap();
        // let dick: unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT = transmute(&shitter);
        let dick: HOOKPROC = transmute(shitter);
        HOOK = match SetWindowsHookExW(WH_CALLWNDPROC, dick, dll_module, gui_tid) {

        // Doesn't work when injected
        // Works when LoadLibrary is used
        // HOOK = match SetWindowsHookExW(WH_CALLWNDPROC, Some(callback), dll_module, gui_tid) {

        // Doesn't work when injected
        // Works when LoadLibrary is used
        // HOOK = match SetWindowsHookExW(WH_CALLWNDPROC, Some(callback), dll_module, GetCurrentThreadId()) {

        // HOOK = match SetWindowsHookExW(WH_CALLWNDPROC, HOOKPROC::Some(callback), 0, gui_tid) {
            Ok(handle) => handle,
            Err(_) => {
                info!("Error in SetWindowsHookEx");
                return
            }
        };

        info!("Farty");
        MessageBoxA(HWND(0), s!("Woo!"), s!("hello_dll"), Default::default());

        // Get handle to main window
        // let hwnd = find_window_by_pid(pid).unwrap();

        // if SetWindowSubclass(hwnd, Some(subclass_proc), 0, 0,).as_bool() {
        //     MessageBoxA(HWND(0), s!("Woo!"), s!("hello_dll"), Default::default());
        // }
        // else {
        //     MessageBoxA(HWND(0), s!("Fail!"), s!("hello_dll"), Default::default());
        // }
    };
}

#[no_mangle]
unsafe extern "system" fn callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    info!("Callback triggered! {}", GetCurrentThreadId());
    if HC_ACTION as i32 == n_code {
        let origin = w_param.0 as u32;
        info!("by: {}", origin);
        let fuckle = unsafe { *(l_param.0 as *const CWPSTRUCT) };

        // info!("here: {}", fuckle.message);

        if fuckle.message == WM_SIZING {
            info!("Received WM_PAINT");
        }
    }

    CallNextHookEx(HHOOK::default(), n_code, w_param, l_param)
}

// https://github.com/alexrsagen/binarymagic-tas/blob/517a0dce912192aadee75e0a147abf71d931c016/src/sys.rs
pub fn find_window_by_pid(pid: u32) -> Result<HWND> {
    let mut data = MaybeUninit::new(EnumWindowsData {
        wanted_pid: pid,
        handle: HWND::default(),
        found: false,
    });
    let data = unsafe {
        EnumWindows(
            Some(enum_windows_callback),
            LPARAM(data.as_mut_ptr() as isize),
        );
        data.assume_init()
    };
    if !data.found {
        return Err(Error::new(
            ErrorKind::NotFound,
            "no window found matching pid",
        ));
    }
    Ok(data.handle)
}

#[derive(Default)]
struct EnumWindowsData {
    wanted_pid: u32,
    handle: HWND,
    found: bool,
}

unsafe extern "system" fn enum_windows_callback(handle: HWND, lparam: LPARAM) -> BOOL {
    let data = lparam.0 as *mut EnumWindowsData;
    let mut data = data.as_mut().unwrap();

    let mut pid = MaybeUninit::<u32>::zeroed();
    GetWindowThreadProcessId(handle, Some(pid.as_mut_ptr()));
    let pid = pid.assume_init();

    if pid == data.wanted_pid
        && GetWindow(handle, GW_OWNER).0 == 0
        && IsWindowVisible(handle).as_bool()
    {
        data.handle = handle;
        data.found = true;
        return BOOL(0);
    }

    BOOL(1)
}

extern "system" fn subclass_proc(window: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM, _subclass_id: usize, _ref_data: usize) -> LRESULT {
    if msg == WM_PAINT {
        unsafe {
            // MessageBoxA(HWND(0), s!("ZOMG!"), s!("hello_dll"), Default::default());
            let mut msg =  String::from("ZOMG!");
            let mut ps = PAINTSTRUCT::default();
            let psp = &mut ps as *mut PAINTSTRUCT;
            let rectp = &mut ps.rcPaint as *mut RECT;
            let hdc = BeginPaint(window, psp);
            let brush = CreateSolidBrush(COLORREF(0x0000F0F0)); // yellow
            // All painting occurs here, between BeginPaint and EndPaint.
            FillRect(hdc, &ps.rcPaint, brush);
            DrawTextA(hdc,
                msg.as_bytes_mut(),
                rectp,
                DT_SINGLELINE | DT_CENTER | DT_VCENTER
            );
            EndPaint(window, &ps);
        }

        return LRESULT(0);
    }

    unsafe { DefSubclassProc(window, msg, wparam, lparam) }
}
