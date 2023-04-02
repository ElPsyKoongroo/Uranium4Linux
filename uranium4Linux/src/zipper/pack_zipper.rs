use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use zip::{result::ZipResult, write::FileOptions, ZipWriter};

use crate::checker::{check, check_panic, dlog};
use crate::code_functions::fix_path;
use crate::variables::constants;

use crate::zipper::uranium_structs::FileType;
use super::uranium_structs::UraniumFile;

pub fn compress_pack(name: &str, path: &str, raw_mods: &[String]) -> ZipResult<()> {
    let path = &fix_path(path);

    let zip_file = File::create(name.to_owned() + constants::EXTENSION).unwrap();
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.add_directory("overrides", options).unwrap();

    zip.add_directory("overrides/config", options).unwrap();
    let mut config_files: Vec<UraniumFile> = Vec::new();

    // Iter through all the files and sub-directories in "config/" and set the file type.
    search_files(path, "config/", &mut config_files);

    add_files_to_zip(path, &mut config_files, &mut zip, options);

    // Add the modpack_temp.json file
    let modpack_json = File::open(constants::RINTH_JSON).unwrap();
    let modpack_bytes = modpack_json.bytes().flatten().collect::<Vec<u8>>();

    // Add the hardcoded .jar mods
    add_raw_mods(path, &mut zip, raw_mods, options);

    // Finally add the modpack.json file
    zip.start_file(constants::RINTH_JSON, options)?;
    zip.write_all(&modpack_bytes)?;
    zip.finish()?;

    Ok(())
}

fn search_files(minecraft_path: &str, relative_path: &str, config_files: &mut Vec<UraniumFile>) {
    // Get this directory files
    let mut sub_config_files =
        get_new_files(&(minecraft_path.to_owned() + relative_path), relative_path);

    // Go through the sub_config_files vector and set the right tipe to each file. Then add them to config_files
    for config_file in &mut sub_config_files {
        let path: PathBuf = (minecraft_path.to_owned() + &config_file.get_absolute_path()).into();
        if Path::is_file(&path) {
            (*config_file).set_type(FileType::Data);
            config_files.push(config_file.clone());
        } else {
            (*config_file).set_type(FileType::Dir);
            config_files.push(config_file.clone());
            let new_path = relative_path.to_owned() + &config_file.get_name() + "/";
            search_files(minecraft_path, &new_path, config_files);
        }
    }
}

fn get_new_files(path: &str, relative_path: &str) -> Vec<UraniumFile> {
    let sub_directory = std::fs::read_dir(path);
    let sub_directory = check(sub_directory, true, format!("Error al leer {}", path)).unwrap();

    let sub_config_files: Vec<UraniumFile> = sub_directory
        .map(|file| {
            UraniumFile::new(
                relative_path,
                file.unwrap().file_name().to_str().unwrap(),
                FileType::Other,
            )
        })
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
    file: &mut UraniumFile,
) {
    match file.get_type() {
        FileType::Data => {
            let absolute_path = PathBuf::from(root_path.to_owned() + &file.get_absolute_path());
            let rel_path = "overrides/".to_owned() + &file.get_absolute_path();
            append_config_file(&absolute_path, &rel_path, zip, options);
        }

        FileType::Dir => {
            zip.add_directory(
                "overrides/".to_owned() + &file.get_path() + &file.get_name(),
                options,
            )
            .unwrap();
        }

        FileType::Other => {}
    }
}

fn append_config_file(
    absolute_path: &PathBuf,
    rel_path: &str,
    zip: &mut ZipWriter<File>,
    option: FileOptions,
) {
    // Read the file
    let file = check_panic(
        File::open(absolute_path),
        false,
        format!("zipper; Unable to open {:?}", absolute_path),
    );
    let buffer = file.bytes().flatten().collect::<Vec<u8>>();

    // Is a recoverable error reading 0 bytes from file ?
    // In this case Uranium will just send a warning about it
    // and dont add the file
    if buffer.is_empty() {
        dlog("No bytes readed from the pack");
        return;
    }

    // Add the file to the zip
    check_panic(
        zip.start_file(rel_path, option),
        false,
        format!("zipper; Unable to start zip file {}", rel_path),
    );
    check_panic(
        zip.write_all(&buffer),
        false,
        "zipper; Error while writing zip file {}",
    );
}

fn add_raw_mods(path: &str, zip: &mut ZipWriter<File>, raw_mods: &[String], options: FileOptions) {
    check_panic(
        zip.add_directory("overrides/mods", options),
        false,
        "zipper; Error adding raw mods dir",
    );

    for jar_file in raw_mods {
        let file_name = "overrides/mods/".to_owned() + jar_file;

        #[cfg(debug_assertions)]
        println!("Adding {}", file_name);

        println!("{}", (path.to_owned() + "mods/") + jar_file);
        let buffer = check_panic(
            std::fs::read((path.to_owned() + "mods/") + jar_file),
            false,
            format!("zipper; Unable to read {}", jar_file),
        );

        check_panic(
            zip.start_file(file_name, options),
            false,
            "zipper; Error starting a file in .zip",
        );

        check_panic(
            zip.write_all(&buffer),
            false,
            format!("Error while raw adding {}", jar_file),
        );
    }
}
