use serde::Serialize;
use std::process::Command;
use std::time::{Duration, Instant};
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
    status: String,
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
            thread::sleep(Duration::from_secs(2));

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
        let ping_address = if cfg!(target_os = "windows") {
            "127.0.0.1"
        } else {
            "8.8.8.8"
        };

        loop {
            let latency = ping_host(&ping_address);
            let status = match latency {
                0 => "offline".to_string(),
                1..=50 => "excellent".to_string(),
                51..=100 => "good".to_string(),
                101..=200 => "average".to_string(),
                201..=500 => "poor".to_string(),
                _ => "timeout".to_string(),
            };

            manager
                .emit("ping_update", PingResult { latency, status })
                .unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn ping_host(address: &str) -> u64 {
    let _start_time = Instant::now();
    let timeout_ms = 1000;

    if cfg!(target_os = "windows") {
        let output = Command::new("ping")
            .args(&["-n", "1", "-w", &timeout_ms.to_string(), address])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str
                    .lines()
                    .find(|line| line.contains("time="))
                    .and_then(|line| {
                        line.split("time=")
                            .nth(1)
                            .and_then(|part| part.split('m').next())
                            .and_then(|time| time.parse::<u64>().ok())
                    })
                    .unwrap_or(0)
            }
            _ => 0,
        }
    } else {
        let output = Command::new("ping")
            .args(&["-c", "1", "-W", &(timeout_ms / 1000).to_string(), address])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str
                    .lines()
                    .find(|line| line.contains("time="))
                    .and_then(|line| {
                        line.split("time=")
                            .nth(1)
                            .and_then(|part| part.split(' ').next())
                            .and_then(|time| time.parse::<f64>().ok())
                            .map(|time| time.round() as u64)
                    })
                    .unwrap_or(0)
            }
            _ => 0,
        }
    }
}
