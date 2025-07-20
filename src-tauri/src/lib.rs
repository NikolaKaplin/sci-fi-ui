mod handlers;
mod utils;
use handlers::ai::audio::generate_audio;
use handlers::events::{
    start_cpu_monitor, start_memory_monitor, start_ping_monitor, start_process_monitor,
};
use handlers::term::{
    async_create_shell, async_read_from_pty, async_resize_pty, async_write_to_pty,
};
use portable_pty::{native_pty_system, PtyPair, PtySize};
use std::io::{Read, Write};
use std::{io::BufReader, sync::Arc};
use tauri::async_runtime::Mutex as AsyncMutex;
use tauri::{ Manager};

pub struct AppState {
    pub pty_pair: Arc<AsyncMutex<PtyPair>>,
    pub writer: Arc<AsyncMutex<Box<dyn Write + Send>>>,
    pub reader: Arc<AsyncMutex<BufReader<Box<dyn Read + Send>>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let pty_system = native_pty_system();
    let pty_pair = pty_system
        .openpty(PtySize {
            rows: 40,
            cols: 40,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    let reader = pty_pair.master.try_clone_reader().unwrap();
    let writer = pty_pair.master.take_writer().unwrap();

    tauri::Builder::default()
        .manage(AppState {
            pty_pair: Arc::new(AsyncMutex::new(pty_pair)),
            writer: Arc::new(AsyncMutex::new(writer)),
            reader: Arc::new(AsyncMutex::new(BufReader::new(reader))),
        })
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();

            let handle = app.handle();

            start_cpu_monitor(handle.clone());
            start_memory_monitor(handle.clone());
            start_process_monitor(handle.clone());
            start_ping_monitor(handle.clone());

            let _app_handle = app.handle();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(1)); // Задержка 1 секунда
                main_window.show().unwrap();
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            generate_audio,
            async_write_to_pty,
            async_resize_pty,
            async_create_shell,
            async_read_from_pty
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
