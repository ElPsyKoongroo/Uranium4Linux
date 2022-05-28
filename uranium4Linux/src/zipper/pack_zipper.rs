#![allow(unused)]
use std::{io::{Write, Read}, fs::{File, ReadDir, DirEntry}, path::{Path, PathBuf}, borrow::Borrow};
use zip::{write::FileOptions, ZipWriter, result::ZipResult};
use crate::zipper::uranium_structs::FileType;
use crate::checker;
use super::uranium_structs::UraniumFile;


pub fn compress_pack(name: &str, path: &str) -> ZipResult<()>{
    let zip_file = File::create(name.to_owned() + ".zip").unwrap();
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let config_dir = std::fs::read_dir(path.to_owned() + "config/").unwrap();
    
    zip.add_directory("config", options);
    let mut config_files: Vec<UraniumFile> = config_dir.map(
        |file|
        UraniumFile::new(&(path.to_owned() + "config/"), file.unwrap().file_name().to_str().unwrap(), FileType::Other)
    ).collect();

    // Iter through all the files and sub-directories in "config/" and set the file type.
    search_files(path, "config/", &mut config_files);

    
    // Add the files to modpack.zip
    write_zip(path, &mut config_files, &mut zip, options);
    
    // Finally add the modpack_temp.json file
    let modpack_json = File::open(name.to_owned() + "_temp.json").unwrap();
    let modpack_bytes = modpack_json.bytes().flatten().collect::<Vec<u8>>();

    // Write the zip file
    zip.start_file(name.to_owned() + ".json", options)?;
    zip.write(&modpack_bytes);
    zip.finish()?;

    Ok(())
}

fn search_files(root_path: &str, relative_path: &str, config_files: &mut Vec<UraniumFile>){
    let sub_directory = std::fs::read_dir(root_path.to_owned() + relative_path).unwrap();
    let mut sub_config_files: Vec<UraniumFile> = sub_directory.map(
        |file|
        UraniumFile::new(relative_path, file.unwrap().file_name().to_str().unwrap(), FileType::Other)
    ).collect();
   

    // Go through the sub_config_files vector and set the right tipe to each file. Then add them to config_files
    for config_file in sub_config_files.iter_mut() {
        let path: PathBuf = (root_path.to_owned() + &config_file.get_absolute_path()).into();
        if Path::is_file(&path){
            (*config_file).set_type(FileType::Data);
            config_files.push(config_file.to_owned());
        } else {
            (*config_file).set_type(FileType::Dir);
            config_files.push(config_file.to_owned());
            let new_path = relative_path.to_owned() + &config_file.get_name() + "/";
            search_files(root_path, &new_path, config_files);
        }
    }
}

fn write_zip(root_path: &str, config_files: &mut Vec<UraniumFile>, zip: &mut ZipWriter<File>, options: FileOptions){
    for file in config_files {
        match file.get_type() {
            FileType::Data => {
                let absolute_path = PathBuf::from(root_path.to_owned() + &file.get_absolute_path());
                let rel_path = file.get_absolute_path();
                append_config_file(absolute_path, &rel_path, zip, options);
            },

            FileType::Dir => {
                zip.add_directory(file.get_path() + &file.get_name(), options);
            },

            _ => {}
        }
    }
}

fn append_config_file(absolute_path: PathBuf, rel_path: &str, zip: &mut ZipWriter<File>, option: FileOptions) {
    // Read the file
    let mut file = File::open(&absolute_path).unwrap();
    let mut buffer = file.bytes().flatten().collect::<Vec<u8>>();
    
    // Add the file to the zip
    zip.start_file(rel_path, option).unwrap();
    checker::check(zip.write(&buffer), false, false, "Error while writing");
}