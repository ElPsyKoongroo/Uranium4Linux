use std::{fmt::Debug, str::FromStr};

use crate::{downloaders::updater::update_modpack, variables::constants::*};

pub async fn update(path: &str) {
    update_modpack(path).await;
}

///Add '/' at the end of the path if it isnt already in it.
pub fn fix_path(path: &str) -> String {
    if !path.ends_with('/') {
        return path.to_owned() + "/";
    }
    path.to_owned()
}


pub fn N_THREADS() -> usize {
    match NTHREADS.read() {
        Ok(e) => *e,
        Err(_) => DEFAULT_NTHREADS,
    }
}

pub fn get_bool_element(args: &[String], flag: &str) -> bool {
    args.iter().any(|f| f == flag)
}

pub fn get_parse_element<T>(args: &[String], flag: &str) -> Option<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    args.iter()
        .position(|f| f == flag)
        .map(|index| args[index + 1].parse().unwrap())
}
