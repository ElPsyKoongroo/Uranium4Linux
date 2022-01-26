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

<<<<<<< HEAD
=======
pub fn input_string(msg: &str, default: String) -> String{
    let mut s= String::new();
    let result: String;
    print!("{}", msg);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let trimmed = s.trim();
    match trimmed.is_empty(){
        true => result = default,
        false => result = trimmed.to_string(),
    }
    result
}

>>>>>>> 5450e3410108f2e9926b8429f9c39a1471cde3e5
