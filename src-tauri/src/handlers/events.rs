use serde::Serialize;
use std::time::Duration;
use std::{ffi::OsStr, thread};
use sysinfo::System;
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Clone)]
struct CpuUsage {
    usage: f32,
}

#[derive(Serialize, Clone)]
struct MemoryUsage {
    used: u64,
    total: u64,
    percentage: f64,
}

#[derive(Serialize)]
struct ProcessInfo<'a> {
    name: &'a OsStr,
    pid: u32,
    cpu: f32,
    memory: u64,
}

#[derive(Serialize, Clone)]
struct PingResult {
    latency: u64,
}

pub fn start_cpu_monitor(manager: AppHandle) {
    thread::spawn(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu_all();
            thread::sleep(Duration::from_secs(1));

            let usage = sys.global_cpu_usage();
            manager.emit("cpu_update", CpuUsage { usage }).unwrap();
        }
    });
}

pub fn start_memory_monitor(manager: AppHandle) {
    thread::spawn(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_memory();
            thread::sleep(Duration::from_secs(1));

            let total = sys.total_memory();
            let used = sys.used_memory();
            let percentage = (used as f64 / total as f64) * 100.0;

            manager
                .emit(
                    "memory_update",
                    MemoryUsage {
                        used,
                        total,
                        percentage,
                    },
                )
                .unwrap();
        }
    });
}

pub fn start_process_monitor(manager: AppHandle) {
    thread::spawn(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_all();
            thread::sleep(Duration::from_secs(2)); // Реже обновляем процессы

            let processes: Vec<ProcessInfo> = sys
                .processes()
                .iter()
                .map(|(_, p)| ProcessInfo {
                    name: p.name(),
                    pid: p.pid().as_u32(),
                    cpu: p.cpu_usage(),
                    memory: p.memory(),
                })
                .collect();

            manager.emit("process_update", &processes).unwrap();
        }
    });
}

pub fn start_ping_monitor(manager: AppHandle) {
    thread::spawn(move || {
        loop {
            let latency = 100; // Здесь должна быть реальная логика ping
            thread::sleep(Duration::from_secs(1));

            manager.emit("ping_update", PingResult { latency }).unwrap();
        }
    });
}
