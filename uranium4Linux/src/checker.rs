use chrono::prelude::*;
use once_cell::sync::Lazy;
use std::{
    fmt::Debug,
    io::{BufWriter, Write},
    sync::RwLock,
};

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
/// - If T => Return Result<T, E>
/// - If E => check will log the error in log.txt and returns Result<T, E>
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

/// # Given a Result<T, E> it checks if Ok() or Err()
/// - If T => Return T
/// - If E => check will log the error in log.txt and returns T::default()
///
/// # Panics
///
///  This function NEVER panics
#[allow(unused)]
pub fn check_default<
    T: std::default::Default,
    E,
    M: std::fmt::Display + std::convert::AsRef<[u8]>,
>(
    value: Result<T, E>,
    show_error: bool,
    msg: M,
) -> T
where
    E: Debug,
{
    match value {
        Ok(e) => e,
        Err(ref error) => {
            manage_error(error, show_error, msg);
            T::default()
        }
    }
}

/// # LOG
/// This function will write in the log file the messages.
///
/// # Panics
/// This function NEVER panics
pub fn log<M: std::fmt::Display>(msg: M) {
    let mut guard = LOG_FILE.write().unwrap();
    let log_msg = format!("[LOG] {}\n", msg);
    check(
        guard.write_all(log_msg.as_bytes()),
        false,
        "log; Failed to log",
    )
    .ok();
    guard.flush().unwrap();
}

/// # OLOG
/// This function will write in the log file the messages and also will print it in stdout
///
/// # Panics
/// This function NEVER panics
pub fn olog<M: std::fmt::Display>(msg: M) {
    let mut guard = LOG_FILE.write().unwrap();
    let log_msg = format!("[LOG] {}\n", msg);
    println!("{log_msg}");
    check(
        guard.write_all(log_msg.as_bytes()),
        false,
        "log; Failed to log",
    )
    .ok();
    guard.flush().unwrap();
}

/// # DLOG
/// This function will write in the log file the messages if debug assertions are enable
///
/// # Panics
/// This function NEVER panics
#[inline]
pub fn dlog<M: std::fmt::Display>(msg: M) {
    #[cfg(debug_assertions)]
    {
        let mut guard = LOG_FILE.write().unwrap();
        let log_msg = format!("[LOG] {}\n", msg);
        check(
            guard.write_all(log_msg.as_bytes()),
            false,
            "log; Failed to log",
        )
        .ok();
        guard.flush().unwrap();
    }
}

/// # OLOG
/// This function will write in the log file the messages and also will print it in stdout
///
/// # Panics
/// This function NEVER panics
pub fn elog<M: std::fmt::Display>(msg: M) {
    let mut guard = LOG_FILE.write().unwrap();
    let log_msg = format!("[ERROR] {}\n", msg);
    println!("{log_msg}");
    check(
        guard.write_all(log_msg.as_bytes()),
        false,
        "log; Failed to log",
    )
    .ok();
    guard.flush().unwrap();
}

fn manage_error<E, M: std::fmt::Display + std::convert::AsRef<[u8]>>(
    error: E,
    show_error: bool,
    msg: M,
) where
    E: Debug,
{
    let msg = format!("[ERROR] {} {:?}\n", msg, error);
    log(msg);
    if show_error {
        eprintln!("Next error ocurred in runtime: {:?}\n\n", error);
    }
}
