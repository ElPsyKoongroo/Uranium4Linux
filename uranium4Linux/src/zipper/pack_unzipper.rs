use std::fs::{File, remove_dir_all};
use std::fs::{create_dir};
use std::io::{Write, Read};
use tokio::runtime::Builder;
use zip::result::ZipResult;

use crate::checker::check;
use crate::code_functions::download_modpack;
use crate::variables::constants::EXTENSION;
use crate::variables::constants::TEMP_DIR;


pub async fn unzip_pack(file_path: &str, minecraft_root: &str) -> ZipResult<()>{
    println!("File: {}", file_path);

    let json_name = file_path.split("/").last().unwrap().strip_suffix(EXTENSION).unwrap().to_owned() + ".json";

    let zip_file = File::open(file_path).unwrap();

    let mut zip = zip::ZipArchive::new(zip_file)?;

    check(
        create_dir(TEMP_DIR),
        true,
        true,
        &format!("Error al crear el directorio temporal {} {}", "unzipper", 22)
    );

    check(
        zip.extract(TEMP_DIR),
        true,
        true,
        &format!("Error al extraer el modpack: {} {}", "unzipper", 24)

    );

    download_modpack(&(TEMP_DIR.to_owned() + &json_name), minecraft_root).await.unwrap();
    remove_dir_all(TEMP_DIR);

    Ok(())
}


fn full_filename(path: &str, name: &str) -> String {
    path.to_owned() + name
}