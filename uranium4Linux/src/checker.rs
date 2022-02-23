use std::fmt::Debug;
use crate::easy_input;





pub fn check<T, E>(value: Result<T, E>) -> Option<T> 
where E: Debug{

    match value {
        Ok(e) => {
            Some(e)
        }
        Err(error) => {
            println!("\n\nNext error ocurred in runtime: {:?}\n\n", error);
            //let _ = easy_input::input("Press enter to continue...", String::from(""));
            None
        }
    }


}