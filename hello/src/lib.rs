use log::{info};
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls};
use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };
use windows::{ Win32::Graphics::Gdi::*, Win32::UI::WindowsAndMessaging::*, };
use windows::Win32::{UI::Shell::{DefSubclassProc, SetWindowSubclass, RemoveWindowSubclass}};
use windows::Win32::System::Threading::{GetCurrentProcessId, GetCurrentThreadId};
use windows::Win32::UI::WindowsAndMessaging::{ CallNextHookEx, HHOOK, MSG };

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    unsafe {
        match call_reason {
            DLL_PROCESS_ATTACH => attach_process(dll_module),
            DLL_PROCESS_DETACH => info!("Process detach called: {}", GetCurrentProcessId()),
            DLL_THREAD_ATTACH  => info!("Thread attach called: {}", GetCurrentThreadId()),
            DLL_THREAD_DETACH  => info!("Thread detach called: {}", GetCurrentThreadId()),
            _ => ()
        }
    }

    true
}

fn attach_process(dll_module: HINSTANCE) {
    eventlog::init("Example Log", log::Level::Info).unwrap();

    unsafe {
        info!("hello.dll loaded by process: {}", GetCurrentProcessId());
        DisableThreadLibraryCalls(dll_module);
    }
}

#[no_mangle]
unsafe extern "system" fn CallWndProc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if HC_ACTION as i32 == n_code {
        let origin = w_param.0 as u32;
        let param = unsafe { *(l_param.0 as *const CWPSTRUCT) };

        match param.message {
            WM_SIZING => info!("CallWndProc: Received WM_SIZING"),
            WM_PAINT => {
                info!("CallWndProc: Received WM_PAINT");
                SetWindowSubclass(param.hwnd, Some(Subclassproc), 0, 0);
            },

            _ => ()
        };
    }

    CallNextHookEx(HHOOK::default(), n_code, w_param, l_param)
}

#[no_mangle]
unsafe extern "system" fn GetMsgProc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if HC_ACTION as i32 == n_code {
        let origin = w_param.0 as u32;
        let param = unsafe { *(l_param.0 as *const MSG) };

        match param.message {
            WM_SIZING => info!("GetMsgProc: Received WM_SIZING"),
            WM_PAINT => {
                info!("GetMsgProc: Received WM_PAINT");
                SetWindowSubclass(param.hwnd, Some(Subclassproc), 0, 0);
            },

            _ => ()
        };
    }

    CallNextHookEx(HHOOK::default(), n_code, w_param, l_param)
}

#[no_mangle]
extern "system" fn Subclassproc(window: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM, _subclass_id: usize, _ref_data: usize) -> LRESULT {
    unsafe { RemoveWindowSubclass(window, Some(Subclassproc), 0); }

    match msg {
        WM_PAINT => unsafe {
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

            return LRESULT(0);
        },
        _ => ()
    };

    unsafe { DefSubclassProc(window, msg, wparam, lparam) }
}
