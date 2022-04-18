use std::{path::Path, error::Error};
use crate::modpack_loader::loader::ModPackDownloader;





pub async fn download_modpack(modpack: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
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