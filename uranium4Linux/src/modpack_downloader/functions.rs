use crate::checker::check;
use crate::variables::constants::TEMP_DIR;
use reqwest::Response;
use std::fs;
use tokio::task::JoinHandle;

pub async fn write_mod(path: &str, res: Response, name: &str) {
    let web_res = res;
    let full_path = path.to_owned() + name;
    let content = web_res.bytes().await.unwrap();

    match tokio::fs::write(&full_path, content).await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", full_path);
            eprintln!("{}", e);
        }
    }
}

pub async fn get_writters(
    responses: Vec<Response>,
    names: Vec<String>,
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

    // Iter through the override directory and copy the content to
    // Minecraft Root (`destination_path`)
    for file in fs::read_dir(&overrides_folder).unwrap().flatten() {
        if file.file_type().unwrap().is_dir() {
            fs_extra::dir::copy(file.path(), destination_path, &options).unwrap();
        } else {
            let copy_status = fs_extra::file::copy(file.path(), destination_path, &file_options);
            check(copy_status, false, false, "");
        }
         
    }
}
