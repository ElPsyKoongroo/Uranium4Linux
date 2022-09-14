use crate::modpack_downloader::{loader::ModPackDownloader, updater::update_modpack};
use crate::variables::constants::*;
use std::{fmt::Debug, path::Path, str::FromStr};

pub async fn download_modpack<'a>(
    modpack: &str,
    path: &'a str,
    n_threads: usize,
) -> Result<(), usize> {
    if !Path::new(path).exists() {
        return Err(1);
    };

    let mut modpack_loader: ModPackDownloader;
    if n_threads == 0 {
        modpack_loader = ModPackDownloader::new();
    } else {
        modpack_loader = ModPackDownloader::new_with_threads(n_threads);
    }

    modpack_loader.set_path(String::from(path));
    modpack_loader.load_pack(modpack);
    modpack_loader.start().await.unwrap();
    Ok(())
}

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

pub fn get_bool_element(args: &[String], flag: &str) -> bool {
    args.iter().any(|f| f == flag)
}

pub fn N_THREADS() -> usize {
    match NTHREADS.read() {
        Ok(e) => *e,
        Err(_) => DEFAULT_NTHREADS,
    }
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
