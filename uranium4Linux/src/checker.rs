use std::{fmt::Debug, process::exit};

/// Given a Result<T, E> it checks if Ok() or Err(), 
/// if Ok it returns T
/// if Err it calls manage_error
pub fn check<T, E>(value: Result<T, E>, panic: bool, show_error: bool, msg: &str) -> Option<T>
where
    E: Debug,
{
    match value {
        Ok(e) => Some(e),
        Err(error) => {
            manage_error(error, show_error, msg);
            if !panic {
                None
            } else {
                exit(0);
            }
        }
    }
}

fn manage_error<E>(error: E, show_error: bool, msg: &str)
where
    E: Debug,
{
    eprintln!("{msg}");
    if show_error {
        eprintln!("Next error ocurred in runtime: {:?}\n\n", error);
    }
}
