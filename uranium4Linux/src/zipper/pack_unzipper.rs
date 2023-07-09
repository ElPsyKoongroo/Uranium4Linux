use std::{
    fs::{create_dir, remove_dir_all, File},
    path::Path,
};

use crate::{
    checker::{check, check_panic},
    error::ModpackError,
    variables::constants::TEMP_DIR,
};

pub fn unzip_temp_pack<I: AsRef<Path>>(file_path: I) -> Result<(), ModpackError> {
    // Should't fail, in case this fail the program must end because the
    // file_path is wrong or the file is not valid
    let zip_file = check(
        File::open(file_path.as_ref()),
        true,
        format!("unzipper; Zip file not found! {:?}", file_path.as_ref()),
    )
    .map_err(|_| ModpackError::FileNotFound)?;

    let mut zip = zip::ZipArchive::new(zip_file).map_err(|_| ModpackError::WrongFileFormat)?;

    let a = check(
        create_dir(TEMP_DIR),
        false,
        "unzipper; Could not create temporal dir",
    );
    if a.is_err() {
        remove_temp_pack();
    }

    check_panic(
        zip.extract(TEMP_DIR),
        true,
        "unzipper; Error while extracting the modpack",
    );

    Ok(())
}

pub fn remove_temp_pack() {
    check_panic(
        remove_dir_all(TEMP_DIR),
        false,
        "unzipper; Error at deleting temp dir",
    );
}
