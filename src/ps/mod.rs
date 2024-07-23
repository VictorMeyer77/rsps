use crate::ps::error::Error;
use std::process::Output;

pub mod error;
pub mod macos;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Process {
    pid: u32,        // Process ID
    ppid: u32,       // Parent Process ID
    uid: u32,        // User ID of the process owner
    lstart: i64,     // Exact date and time when the process started
    pcpu: f32,       // CPU usage percentage
    pmem: f32,       // Memory usage percentage
    status: String,  // Process status
    command: String, // Command with all its arguments
}

pub trait Ps {
    fn os_command() -> Result<Output, Error>;
    fn parse_output(output: &str) -> Result<Vec<Process>, Error> {
        let mut processes: Vec<Process> = vec![];
        for row in output.lines().skip(1) {
            if let Ok(process) = Self::parse_row(row) {
                processes.push(process)
            } else {
                println!("------- Invalid row {}", row) // todo replace log
            }
        }
        Ok(processes)
    }
    fn parse_row(row: &str) -> Result<Process, Error>;
    fn parse_date(date_chunks: &[&str]) -> Result<i64, Error>;
    fn exec() -> Result<Vec<Process>, Error> {
        let output = Self::os_command()?;
        Ok(Self::parse_output(&String::from_utf8_lossy(
            &output.stdout,
        ))?)
    }
    fn display(processes: Vec<Process>) {
        println!(
            "{0: <6} | {1: <5} | {2: <5} | {3: <10} | {4: <5} | {5: <5} | {6: <6} | {7: <5}",
            "pid", "ppid", "uid", "lstart", "pcpu", "pmem", "status", "command"
        );
        processes.iter().for_each(|process| {
            let truncate_command = if process.command.len() < 100 {
                &process.command
            } else {
                &process.command[..100]
            };
            println!(
                "{0: <6} | {1: <5} | {2: <5} | {3: <10} | {4: <5} | {5: <5} | {6: <6} | {7: <5}",
                process.pid,
                process.ppid,
                process.uid,
                process.lstart,
                process.pcpu,
                process.pmem,
                process.status,
                truncate_command
            );
        });
    }
}
