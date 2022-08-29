use std::{path::Path, str::FromStr, fmt::Debug};
use crate::variables::constants::*;
use crate::modpack_downloader::{loader::ModPackDownloader, updater::update_modpack};

pub async fn download_modpack<'a>(modpack: &str, path: &'a str, n_threads: usize) -> Result<(), usize> {
    if !Path::new(path).exists() {
        return Err(1)
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

pub async fn update(path: &str){
    update_modpack(path).await;
}

///Add '/' at the end of the path if it isnt already in it. 
pub fn fix_path(path: &str) -> String{
    if !path.ends_with('/') {
        return path.to_owned() + "/"
    }
    path.to_owned()
}

pub fn get_bool_element(args: &Vec<String>, flag: &str) -> bool { 
    match args.iter().position(|f| f == flag) {
        Some(index) => true,
        None => false,
    }
}

pub fn N_THREADS() -> usize {
    match NTHREADS.read() {
        Ok(e) => *e,
        Err(_) => DEFAULT_NTHREADS
    }    
}

pub fn get_parse_element<T: FromStr>(args: &Vec<String>, flag: &str) -> Option<T>  
where T: FromStr, <T as FromStr>::Err: Debug {
    match args.iter().position(|f| f == flag) {
        Some(index) => Some(args[index + 1].parse().unwrap()),
        None => None,
    }
}
