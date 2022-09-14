use std::io::{stdin, stdout, Write};
use std::str::FromStr;

pub fn input<T>(msg: &str, default: T) -> T
where
    T: FromStr,
{
    let mut s = String::new();
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let trimmed = s.trim();
    match trimmed.parse::<T>() {
        Ok(i) => i,
        Err(_) => default,
    }
}
