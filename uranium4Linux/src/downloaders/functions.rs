use log::error;

use crate::variables::constants::TEMP_DIR;
use std::{fs, path::{PathBuf, Path}};

pub fn overrides(destination_path: &Path, overrides_folder: &str) {
    // Copy all the content of overrides into the minecraft root folder
    let options = fs_extra::dir::CopyOptions::new();
    let mut file_options = fs_extra::file::CopyOptions::new();
    file_options.overwrite = true;
    let overrides_folder = TEMP_DIR.to_owned() + overrides_folder;

    let entries = match fs::read_dir(&overrides_folder) {
        Ok(e) => e,
        Err(error) => {
            // We dont care about this result, we are going to panic or just leave
            // this function in case there is an error so no need to manage it
            error!("Error reading overrides folder: {}", error);
            // TODO! Fix this panic. Make the function return a result
            // and manage (or not) the error in parent functions
            panic!();
            // return;
        }
    };

    // Iter through the override directory and copy the content to
    // Minecraft Root (`destination_path`)
    for file in entries.flatten() {
        // There's no need to panick, ¿Is this a mess?
        // TODO! Check if file_type can actually panic here.
        if file.file_type().unwrap().is_dir() {
            let _ = fs_extra::dir::copy(file.path(), destination_path, &options);
        } else {
            let _ = std::fs::copy(&file.path(), destination_path.join(&file.file_name()));
        }
    }
}
