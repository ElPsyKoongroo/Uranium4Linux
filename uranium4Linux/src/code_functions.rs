use std::path::Path;
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
