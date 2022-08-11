use crate::checker::check;
use reqwest::Response;
use tokio::task::JoinHandle;
use std::fs;
use crate::variables::constants::TEMP_DIR;

pub async fn write_mod(path: &str, res: Response, name: &str) {
    let web_res = res;
    let full_path = path.to_owned() + name;
    let content = web_res.bytes().await.unwrap();
    tokio::fs::write(full_path, content).await.unwrap();
}

pub async fn get_writters(
    responses: Vec<Response>,
    names: Vec<String>,
    destination_path: &str,
) -> Vec<JoinHandle<()>> {
    let mut writters = Vec::new();
    let mut i = 0;
    for response in responses.into_iter() {
        let path_ref = destination_path.to_owned();
        let mod_name = names[i].clone();

        let task = async move {
            write_mod(&path_ref, response, &mod_name).await;
        };
        writters.push(tokio::spawn(task));
        i += 1;
    }
    writters
}

pub fn overrides(destination_path: &str, overrides_folder: &str){
    // Copy all the content of overrides into the minecraft root folder
    let options = fs_extra::dir::CopyOptions::new();
    let file_options = fs_extra::file::CopyOptions::new();
    let overrides_folder = (TEMP_DIR.to_owned() + overrides_folder).to_owned();


    // Iter through the override directory and copy the content to 
    // Minecraft Root (destination_path)
    for file in fs::read_dir(&overrides_folder).unwrap() {
        match file{
            Ok(e) => {
                if e.file_type().unwrap().is_dir() {
                    fs_extra::dir::copy(
                        e.path(), 
                        destination_path, 
                        &options
                    ).unwrap();

                } else {
                    let copy_status = fs_extra::file::copy(
                        e.path(), 
                        destination_path, 
                        &file_options
                    );
                    check(copy_status, false, false, "");
                }
            }
            Err(_) => {},
        };
    }
}
