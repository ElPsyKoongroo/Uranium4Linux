use std::{fmt::Debug, str::FromStr, path::Path};

use crate::{downloaders::updater::update_modpack, variables::constants::*};

#[allow(unused)]
pub async fn update(path: &Path) {
    update_modpack(path).await;
}

#[allow(non_snake_case)]
/// Returns the actual max threads allowed.
pub fn N_THREADS() -> usize {
    match NTHREADS.read() {
        Ok(e) => *e,
        Err(_) => DEFAULT_NTHREADS,
    }
}

#[allow(unused)]
pub fn get_bool_element(args: &[String], flag: &str) -> bool {
    args.iter().any(|f| f == flag)
}

#[allow(unused)]
pub fn get_parse_element<T>(args: &[String], flag: &str) -> Option<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    args.iter()
        .position(|f| f == flag)
        .map(|index| args[index + 1].parse().unwrap())
}
