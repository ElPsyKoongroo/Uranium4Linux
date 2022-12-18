use std::fs;

use reqwest::Response;
use tokio::task::JoinHandle;

use crate::checker::{check, check_panic};
use crate::variables::constants::TEMP_DIR;

pub async fn write_mod(path: &str, res: Response, name: &str) {
    let full_path = path.to_owned() + name;
    let content = res.bytes().await.unwrap();

    match tokio::fs::write(&full_path, content).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", full_path);
            eprintln!("{}", e);
        }
    }
}

pub async fn get_writters(
    responses: Vec<Response>,
    names: &[String],
    destination_path: &str,
) -> Vec<JoinHandle<()>> {
    let mut writters = Vec::new();
    for (i, response) in responses.into_iter().enumerate() {
        let path_ref = destination_path.to_owned();
        let mod_name = names[i].clone();

        let task = async move {
            write_mod(&path_ref, response, &mod_name).await;
        };
        writters.push(tokio::spawn(task));
    }
    writters
}

pub fn overrides(destination_path: &str, overrides_folder: &str) {
    // Copy all the content of overrides into the minecraft root folder
    let options = fs_extra::dir::CopyOptions::new();
    let file_options = fs_extra::file::CopyOptions::new();
    let overrides_folder = TEMP_DIR.to_owned() + overrides_folder;

    let entries = match fs::read_dir(&overrides_folder) {
        Ok(e) => e,
        Err(error) => {
            // We dont care about this result, we are going to panic or just leave
            // this function in case there is an error so no need to manage it
            #[allow(unused_must_use)]
            {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        check(Err::<(), ()>(()), false, "functions: No overrides folder")
                    }
                    std::io::ErrorKind::PermissionDenied => Ok(check_panic(
                        Err::<(), ()>(()),
                        false,
                        "functions: Permision denied",
                    )),
                    _ => check(
                        Err::<(), ()>(()),
                        false,
                        "functions: Cant specify the error type",
                    ),
                };
            }
            return;
        }
    };

    // Iter through the override directory and copy the content to
    // Minecraft Root (`destination_path`)
    for file in entries.flatten() {
        // There's no need to panick, ¿Is this a mess?
        // TODO! Check if file_type can actually panic here.
        if file.file_type().unwrap().is_dir() {
            let _ = check(
                fs_extra::dir::copy(file.path(), destination_path, &options),
                false,
                "functions: Failt to copy override file",
            );
        } else {
            let copy_status = fs_extra::file::copy(file.path(), destination_path, &file_options);
            check(
                copy_status,
                false,
                &format!("Error coppying {:?}", file.path()),
            )
            .unwrap();
        }
    }
}
