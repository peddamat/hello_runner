use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };
use windows::{ core::*, Win32::UI::WindowsAndMessaging::MessageBoxA, };
use windows::{ Win32::Graphics::Gdi::*, Win32::UI::WindowsAndMessaging::*, };
use windows::Win32::{UI::Shell::{DefSubclassProc, SetWindowSubclass}};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::{ Win32::UI::WindowsAndMessaging::{ EnumWindows, IsWindowVisible, }, };
use std::io::{Error, ErrorKind, Result};
use std::mem::MaybeUninit;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    true
}

fn attach() {
    unsafe {
        let pid = GetCurrentProcessId();

        let hwnd = find_window_by_pid(pid).unwrap();

        if SetWindowSubclass(hwnd, Some(subclass_proc), 0, 0,).as_bool() {
            MessageBoxA(HWND(0), s!("Woo!"), s!("hello_dll"), Default::default());
        }
        else {
            MessageBoxA(HWND(0), s!("Fail!"), s!("hello_dll"), Default::default());
        }
    };
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
