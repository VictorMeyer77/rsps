use crate::ps::error::Error;
use crate::ps::macos::AArch64;
use std::env::consts;
use std::process::Output;

pub mod error;
pub mod macos;

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
                println!("------- Invalid row {}", row) // todo replace log
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
        "macos" => AArch64::exec(),
        _ => Err(Error::Unimplemented {
            os: consts::OS.to_string(),
            arch: consts::ARCH.to_string(),
        }),
    }
}


#[cfg(test)]
mod tests {
    use crate::ps::macos::AArch64;
    use crate::ps::{Ps, rsps};

    #[test]
    fn test() {
        //rsps().unwrap();
        println!("{:?}", AArch64::exec().unwrap());
    }
}
