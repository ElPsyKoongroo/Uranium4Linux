use crate::modpack_loader::updater::*;
use crate::modpack_loader::loader::*;
use std::error::Error;
use std::path::Path;

pub async fn download_modpack(modpack: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(path.as_str()).exists() {
        return Err(Box::<dyn Error>::from(format!(
            "{} is not a valid path !",
            path
        )));
    };
    let mut modpack_loader = ModPackDownloader::new();
    modpack_loader.set_path(String::from(path));
    modpack_loader.load_pack(modpack);
    modpack_loader.start().await?;
    println!("\n\n");
    Ok(())
}

pub async fn update(path: String){
    update_modpack(path).await;
}