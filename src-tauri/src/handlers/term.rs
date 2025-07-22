// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use portable_pty::{CommandBuilder, PtySize};
use std::{
    io::{BufRead, Write},
    process::exit,
    thread::{self},
};

use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn async_create_shell(state: State<'_, AppState>) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let mut cmd = CommandBuilder::new("powershell.exe");

    #[cfg(not(target_os = "windows"))]
    let mut cmd = CommandBuilder::new("bash");

    #[cfg(target_os = "windows")]
    cmd.env("TERM", "cygwin");

    #[cfg(not(target_os = "windows"))]
    cmd.env("TERM", "xterm-256color");

    let mut child = state
        .pty_pair
        .lock()
        .await
        .slave
        .spawn_command(cmd)
        .map_err(|err| err.to_string())?;

    thread::spawn(move || {
        let status = child.wait().unwrap();
        exit(status.exit_code() as i32)
    });
    Ok(())
}

#[tauri::command]
pub async fn async_write_to_pty(data: &str, state: State<'_, AppState>) -> Result<(), ()> {
    write!(state.writer.lock().await, "{}", data).map_err(|_| ())
}

#[tauri::command]
pub async fn async_read_from_pty(state: State<'_, AppState>) -> Result<Option<String>, ()> {
    let mut reader = state.reader.lock().await;
    let data = {
        let data = reader.fill_buf().map_err(|_| ())?;

        if data.len() > 0 {
            std::str::from_utf8(data)
                .map(|v| Some(v.to_string()))
                .map_err(|_| ())?
        } else {
            None
        }
    };

    if let Some(data) = &data {
        reader.consume(data.len());
    }

    println!("{:?}", data);
    Ok(data)
}

#[tauri::command]
pub async fn async_resize_pty(rows: u16, cols: u16, state: State<'_, AppState>) -> Result<(), ()> {
    state
        .pty_pair
        .lock()
        .await
        .master
        .resize(PtySize {
            rows,
            cols,
            ..Default::default()
        })
        .map_err(|_| ())
}
