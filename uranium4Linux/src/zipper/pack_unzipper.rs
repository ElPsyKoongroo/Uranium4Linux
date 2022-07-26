use std::fs::{File, remove_dir_all};
use std::fs::{create_dir};
use zip::result::ZipResult;
use fs_extra;

use crate::checker::check;
use crate::code_functions::download_modpack;
use crate::variables::constants::EXTENSION;
use crate::variables::constants::TEMP_DIR;


pub async fn unzip_pack(file_path: &str, minecraft_root: &str, n_threads: usize) -> ZipResult<()>{
    let json_name = file_path.split("/").last().unwrap().strip_suffix(EXTENSION).unwrap().to_owned() + ".json";

    unzip_temp_pack(file_path);

    
    let options = fs_extra::dir::CopyOptions::new();
    let config_result = fs_extra::dir::copy(
        TEMP_DIR.to_owned() + "config",
        minecraft_root,
        &options
    );
    let raw_mods_result = fs_extra::dir::copy(
        TEMP_DIR.to_owned() + "mods",
        minecraft_root,
        &options
    );
    
    check(config_result, true, true, "No config to copy");
    check(raw_mods_result, true, true, "No raw mods to copy");

    download_modpack(&(TEMP_DIR.to_owned() + &json_name), minecraft_root, n_threads).await.unwrap();
    remove_temp_pack();

    Ok(())
}


pub fn unzip_temp_pack(file_path: &str){
    let zip_file = File::open(file_path).expect("Could not open the zip file");

    // Should't fail, in case this fail the program must end because the file_path is wrong or the
    // file is not valid
    let mut zip = zip::ZipArchive::new(zip_file).unwrap();

    let a: Option<()> = check(
        create_dir(TEMP_DIR),
        false,
        true,
        &format!("Error al crear el directorio temporal {} {}", "unzipper", 22)
    );
    match a {
        None => remove_dir_all(TEMP_DIR).unwrap(),
        Some(_) => {},
    };

    check(
        zip.extract(TEMP_DIR),
        true,
        true,
        &format!("Error al extraer el modpack: {} {}", "unzipper", 24)
    );
}

pub fn remove_temp_pack(){
    check(remove_dir_all(TEMP_DIR), false, true, "Error at deleting temp dir");
}
