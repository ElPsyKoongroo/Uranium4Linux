use super::uranium_structs::UraniumFile;

use crate::code_functions::fix_path;
use crate::checker::check;
use crate::zipper::uranium_structs::FileType;
use crate::{checker, variables::constants};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use zip::{result::ZipResult, write::FileOptions, ZipWriter};

pub fn compress_pack(name: &str, path: &str, raw_mods: Vec<String>) -> ZipResult<()> {

    let path = &fix_path(path);

    let zip_file = File::create(name.to_owned() + constants::EXTENSION).unwrap();
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.add_directory("config", options).unwrap();
    let mut config_files: Vec<UraniumFile> = Vec::new();

    // Iter through all the files and sub-directories in "config/" and set the file type.
    search_files(path, "config/", &mut config_files);

    add_files_to_zip(path, &mut config_files, &mut zip, options);

    // Add the modpack_temp.json file
    let modpack_json = File::open(name.to_owned() + "_temp.json").unwrap();
    let modpack_bytes = modpack_json.bytes().flatten().collect::<Vec<u8>>();

    // Add the hardcoded .jar mods
    add_raw_mods(path, &mut zip, raw_mods, options);

    // Finally add the modpack.json file
    zip.start_file(name.to_owned() + ".json", options)?;
    zip.write(&modpack_bytes)?;
    zip.finish()?;

    Ok(())
}

fn search_files(minecraft_path: &str, relative_path: &str, config_files: &mut Vec<UraniumFile>) {
    // Get this directory files
    let mut sub_config_files =
        get_new_files(&(minecraft_path.to_owned() + relative_path), relative_path);

    // Go through the sub_config_files vector and set the right tipe to each file. Then add them to config_files
    for config_file in sub_config_files.iter_mut() {
        let path: PathBuf = (minecraft_path.to_owned() + &config_file.get_absolute_path()).into();
        if Path::is_file(&path) {
            (*config_file).set_type(FileType::Data);
            config_files.push(config_file.to_owned());
        } else {
            (*config_file).set_type(FileType::Dir);
            config_files.push(config_file.to_owned());
            let new_path = relative_path.to_owned() + &config_file.get_name() + "/";
            search_files(minecraft_path, &new_path, config_files);
        }
    }
}

fn get_new_files(path: &str, relative_path: &str) -> Vec<UraniumFile> {
    let sub_directory = std::fs::read_dir(path);
    let sub_directory = check(
        sub_directory,
        false,
        true,
        &format!("Error al leer {}", path),
    ).unwrap();

    let sub_config_files: Vec<UraniumFile> = sub_directory
        .map(|file| 
            UraniumFile::new(
                relative_path,
                file.unwrap().file_name().to_str().unwrap(),
                FileType::Other,
            )
        )
        .collect();
    sub_config_files
}

fn add_files_to_zip(
    minecraft_path: &str,
    config_files: &mut Vec<UraniumFile>,
    zip: &mut ZipWriter<File>,
    options: FileOptions,
) {
    for file in config_files {
       match_file(minecraft_path, zip, options, file);
    }
}

fn match_file(
    root_path: &str,
    zip: &mut ZipWriter<File>,
    options: FileOptions,
    file: &mut UraniumFile
){
    match file.get_type() {
        FileType::Data => {
            let absolute_path = PathBuf::from(root_path.to_owned() + &file.get_absolute_path());
            let rel_path = file.get_absolute_path();
            append_config_file(absolute_path, &rel_path, zip, options);
        }

        FileType::Dir => {
            zip.add_directory(file.get_path() + &file.get_name(), options)
                .unwrap();
        }

        _ => {}
    }
}

fn append_config_file(
    absolute_path: PathBuf,
    rel_path: &str,
    zip: &mut ZipWriter<File>,
    option: FileOptions,
) {
    // Read the file
    let file = File::open(&absolute_path).unwrap();
    let buffer = file.bytes().flatten().collect::<Vec<u8>>();

    // Add the file to the zip
    zip.start_file(rel_path, option).unwrap();
    checker::check(zip.write_all(&buffer), false, false, "Error while writing");
}

fn add_raw_mods(
    path: &str,
    zip: &mut ZipWriter<File>,
    raw_mods: Vec<String>,
    options: FileOptions,
) {
    zip.add_directory("mods", options).unwrap();
    
    for jar_file in raw_mods.iter() {
        let file_name = "mods/".to_owned() + jar_file;
        
        #[cfg(debug_assertions)]
        println!("Adding {}", file_name);
        
        let buffer = std::fs::read((path.to_owned() + "mods/") + jar_file).unwrap();
        
        zip.start_file(file_name, options).unwrap();
        
        check(
            zip.write_all(&buffer),
            true,
            false,
            &format!("Error while raw adding {}", jar_file)
        );
    }
}
