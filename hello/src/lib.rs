use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };
use windows::{ core::*, Win32::UI::WindowsAndMessaging::MessageBoxA, };

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
        MessageBoxA(HWND(0), s!("ZOMG!"), s!("hello_dll"), Default::default());
    };
}

#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
