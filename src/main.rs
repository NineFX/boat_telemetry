use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time;
use std::time::SystemTime;
use sysinfo::System;

const LOG_PATH: &str = "/var/log/boat_telemetry.log";
const SLEEP_SECONDS: u64 = 60;

#[derive(Debug, Serialize)]
struct ProcessTelemetry {
    name: String,
    pid: u32,
    root: Option<String>,
    num_open_files: Option<usize>,
    memory: u64,
    cpu: f32,
    group_id: Option<u32>,
}

#[derive(Debug, Serialize)]
enum TelemetryEntry {
    ProcessTelemetries(u64, Vec<ProcessTelemetry>),
}
fn main() {
    println!("Starting Boat Telemetry Service");

    let mut open_opts = OpenOptions::new();
    let append_open_opts = open_opts.append(true).create(true);
    let mut log_file = append_open_opts.open(LOG_PATH).unwrap();

    let sleep_duration = time::Duration::from_secs(SLEEP_SECONDS);
    let mut s = System::new_all();

    loop {
        s.refresh_all();
        let process_telemetries_ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let process_telemetries: Vec<ProcessTelemetry> = s
            .processes()
            .iter()
            .map(|(pid, process)| ProcessTelemetry {
                name: process.name().to_string_lossy().to_string(),
                pid: pid.as_u32(),
                root: process.root().map(|p| p.to_string_lossy().to_string()),
                num_open_files: process.open_files(),
                memory: process.memory(),
                cpu: process.cpu_usage(),
                group_id: process.group_id().map(|g| *g), // Dereference the Gid
            })
            .collect();

        let process_telemetries =
            TelemetryEntry::ProcessTelemetries(process_telemetries_ts, process_telemetries);

        // Write each telemetry entry as a JSON line
        let json = serde_json::to_string(&process_telemetries).unwrap();
        writeln!(log_file, "{}", json).unwrap();
        log_file.flush().unwrap();
        thread::sleep(sleep_duration);
    }
}
