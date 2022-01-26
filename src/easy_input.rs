use std::str::FromStr;
use std::io::{stdin,stdout,Write};


pub fn input<T>(msg: &str, default: T) -> T
where T: FromStr{
    let mut s= String::new();
    let result: T;
    print!("{}", msg);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let trimmed = s.trim();
    match trimmed.parse::<T>(){
        Ok(i) => result = i,
        Err(_) => result = default,
    }
    result
}

