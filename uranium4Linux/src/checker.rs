use crate::easy_input;
use std::fmt::Debug;

pub fn check<T, E>(value: Result<T, E>, stop: bool, show_error: bool, msg: &str) -> Option<T>
where
    E: Debug,
{
    match value {
        Ok(e) => Some(e),
        Err(error) => {
            manage_error(error, stop, show_error, msg);
            None
        }
    }
}

fn manage_error<E>(error: E, stop: bool, show_error: bool, msg: &str)
where
    E: Debug,
{
    eprintln!("{msg}");
    if show_error {
        eprintln!("{}\nNext error ocurred in runtime: {:?}\n\n", msg, error);
    }
    if stop {
        let _ = easy_input::input("Press enter to continue...", String::from(""));
    }
}
