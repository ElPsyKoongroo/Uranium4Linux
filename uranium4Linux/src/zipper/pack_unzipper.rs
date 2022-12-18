use std::fs::{File, remove_dir_all};
use std::fs::create_dir;

use crate::checker::{check, check_panic};
use crate::variables::constants::TEMP_DIR;

pub fn unzip_temp_pack(file_path: &str) {
    // Should't fail, in case this fail the program must end because the
    // file_path is wrong or the file is not valid
    let zip_file = check_panic(
        File::open(file_path),
        true,
        "unzipper; Zip file not found! ",
    );

    let mut zip = zip::ZipArchive::new(zip_file).unwrap();

    let a = check(
        create_dir(TEMP_DIR),
        false,
        "unzipper; Could not create temporal dir",
    );
    if let Err(_) = a {
        remove_temp_pack();
    }

    check_panic(
        zip.extract(TEMP_DIR),
        true,
        "unzipper; Error while extracting the modpack",
    );
}

pub fn remove_temp_pack() {
    check_panic(
        remove_dir_all(TEMP_DIR),
        false,
        "unzipper; Error at deleting temp dir",
    );
}
