use crate::easy_input;
use std::fmt::Debug;

pub fn check<T, E>(value: Result<T, E>, stop: bool) -> Option<T>
where
    E: Debug,
{
    match value {
        Ok(e) => Some(e),
        Err(error) => {
            println!("\n\nNext error ocurred in runtime: {:?}\n\n", error);
            if stop {
                let _ = easy_input::input("Press enter to continue...", String::from(""));
            }
            None
        }
    }
}
