use crate::ps::error::Error;
use crate::ps::unix::Unix;
use std::env::consts;
use std::process::Output;
use log::warn;

pub mod error;
pub mod unix;

//todo uniformiser status
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Process {
    pub pid: u32,        // Process ID
    pub ppid: u32,       // Parent Process ID
    pub uid: u32,        // User ID of the process owner
    pub lstart: i64,     // Exact date and time when the process started
    pub pcpu: f32,       // CPU usage percentage
    pub pmem: f32,       // Memory usage percentage
    pub status: String,  // Process status
    pub command: String, // Command with all its arguments
}

pub trait Ps {
    fn os_command() -> Result<Output, Error>;
    fn parse_output(output: &str) -> Result<Vec<Process>, Error> {
        let mut processes: Vec<Process> = vec![];
        for row in output.lines().skip(1) {
            if let Ok(process) = Self::parse_row(row) {
                processes.push(process)
            } else {
                warn!("Process could not be parse {}", row)
            }
        }
        Ok(processes)
    }
    fn parse_row(row: &str) -> Result<Process, Error>;
    fn parse_date(date_chunks: &[&str]) -> Result<i64, Error>;
    fn exec() -> Result<Vec<Process>, Error> {
        let output = Self::os_command()?;
        Self::parse_output(&String::from_utf8_lossy(&output.stdout))
    }
}

pub fn rsps() -> Result<Vec<Process>, Error> {
    match consts::OS {
        _ => Unix::exec()
           // Err(Error::Unimplemented {
           // os: consts::OS.to_string(),
           // arch: consts::ARCH.to_string(),
    }
}
