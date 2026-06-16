// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/native/io.rs
//  Desc:       Native IO
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::io::{self, Write};
use std::cell::RefCell;

#[cfg(windows)]
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
#[cfg(windows)]
use windows_sys::Win32::System::Console::{
    SetConsoleCP, 
    SetConsoleOutputCP, 
    ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    GetStdHandle, 
    GetConsoleMode, 
    SetConsoleMode, 
    STD_OUTPUT_HANDLE
};

thread_local! {
    static STDOUT_HANDLE: RefCell<Option<io::StdoutLock<'static>>> = RefCell::new(None);
    static UTF8_INITIALIZED: RefCell<bool> = RefCell::new(false);
}

fn ensure_utf8_console() {
    UTF8_INITIALIZED.with(|initialized| {
        if !*initialized.borrow() {
            #[cfg(windows)]
            unsafe {
                SetConsoleCP(65001);
                SetConsoleOutputCP(65001);
                let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
                if stdout_handle != INVALID_HANDLE_VALUE {
                    let mut mode: u32 = 0;
                    if GetConsoleMode(stdout_handle, &mut mode) != 0 {
                        SetConsoleMode(stdout_handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
                    }
                }
            }
            *initialized.borrow_mut() = true;
        }
    });
}

fn with_stdout_handle<F, R>(f: F) -> R 
where 
    F: FnOnce(&mut io::StdoutLock<'static>) -> R 
{
    ensure_utf8_console();
    STDOUT_HANDLE.with(|handle| {
        let mut handle = handle.borrow_mut();
        if handle.is_none() {
            let stdout = io::stdout();
            *handle = Some(stdout.lock());
        }
        f(handle.as_mut().unwrap())
    })
}

pub fn print(s: &str) {
    with_stdout_handle(|handle| {
        write!(handle, "{}", s).ok();
    });
}

pub fn println(s: &str) {
    with_stdout_handle(|handle| {
        writeln!(handle, "{}", s).ok();
    });
}

pub fn flush() {
    with_stdout_handle(|handle| {
        handle.flush().ok();
    });
}

pub fn read_line() -> io::Result<String> {
    let mut input = String::new();
    ensure_utf8_console();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn input(prompt: &str) -> io::Result<String> {
    print(prompt);
    flush();
    read_line()
}
