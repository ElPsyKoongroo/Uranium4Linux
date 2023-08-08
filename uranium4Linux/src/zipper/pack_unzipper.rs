use std::{
    fs::{create_dir, remove_dir_all, File},
    path::Path,
};

use crate::{error::ModpackError, variables::constants::TEMP_DIR};
use log::error;

pub fn unzip_temp_pack<I: AsRef<Path>>(file_path: I) -> Result<(), ModpackError> {
    let zip_file = match File::open(file_path.as_ref()) {
        Ok(file) => file,
        Err(e) => {
            error!("Error trying to open the zip file!: {}", e);
            return Err(ModpackError::FileNotFound);
        }
    };

    let mut zip = zip::ZipArchive::new(zip_file).map_err(|_| ModpackError::WrongFileFormat)?;

    if create_dir(TEMP_DIR).is_err() {
        error!("Could not create temporal dir");
        remove_temp_pack();
    }

    if zip.extract(TEMP_DIR).is_err() {
        error!("Error while extracting the modpack");
    }

    Ok(())
}

pub fn remove_temp_pack() {
    if remove_dir_all(TEMP_DIR).is_err() {
        error!("Error at deleting temp dir");
    }
}
