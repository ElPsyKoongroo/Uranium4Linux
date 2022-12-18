use std::fmt::Debug;
use std::io::{BufWriter, Write};
use std::sync::RwLock;

use chrono::prelude::*;
use once_cell::sync::Lazy;

static LOG_FILE: Lazy<RwLock<BufWriter<std::fs::File>>> = Lazy::new(|| {
    RwLock::new(BufWriter::new(
        std::fs::File::create(format!(
            "log_{}.txt",
            Local::now().format("%H-%M-%S_%d-%m-%Y")
        ))
        .unwrap(),
    ))
});

/// # Given a Result<T, E> it checks if Ok() or Err()
/// - If T => Return Result<T, <T, E>
/// - If E => check will log the error in log.txt
///
/// # Panics
///
///  If value is E it will panic 
pub fn check_panic<T, E, M: std::fmt::Display + std::convert::AsRef<[u8]>>(
    value: Result<T, E>,
    show_error: bool,
    msg: M,
) -> T
where
    E: Debug,
{
    match value {
        Ok(val) => val,
        Err(ref error) => {
            manage_error(error, show_error, msg);
            panic!();
        }
    }
}

/// # Given a Result<T, E> it checks if Ok() or Err()
/// - If T => Return Result<T, <T, E>
/// - If E => check will log the error in log.txt
///
/// # Panics
///
///  This function NEVER panics
pub fn check<T, E, M: std::fmt::Display + std::convert::AsRef<[u8]>>(
    value: Result<T, E>,
    show_error: bool,
    msg: M,
) -> Result<T, E>
where
    E: Debug,
{
    match value {
        Ok(ref _e) => value,
        Err(ref error) => {
            manage_error(error, show_error, msg);
            value
        }
    }
}

pub fn log<M: std::fmt::Display + std::convert::AsRef<[u8]>>(msg: M){
    let mut guard = LOG_FILE.write().unwrap();
    let log_msg = format!("[LOG] {}\n", msg);
    check(guard.write_all(log_msg.as_bytes()), false, "log; Failed to log").ok();
    guard.flush().unwrap();
}


fn manage_error<E, M: std::fmt::Display + std::convert::AsRef<[u8]>>(
    error: E,
    show_error: bool,
    msg: M,
) where
    E: Debug,
{
    let mut guard = LOG_FILE.write().unwrap();
    let log_msg = format!("[ERROR] {} {:?}\n", msg, error);
    guard.write_all(log_msg.as_bytes()).expect("Failed to log");
    guard.flush().unwrap();
    if show_error {
        eprintln!("Next error ocurred in runtime: {:?}\n\n", error);
    }
}
