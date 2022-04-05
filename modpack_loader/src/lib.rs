pub mod modpack_loader;

use crate::modpack_loader::loader::*;
use std::error::Error;
use std::path::Path;

#[allow(dead_code)]
pub async fn download_modpack(modpack: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
    // TODO
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

#[tokio::test]
async fn my_test() {
    use crate::modpack_loader::updater::*;
    let path = "C:\\Users\\usuario\\Desktop\\Uranium4Linux\\Modpack_1";

    let a = update_modpack(path.to_string()).await;
    assert!(true);
}
