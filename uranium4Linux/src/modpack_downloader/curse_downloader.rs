use crate::{variables::constants::TEMP_DIR, checker::check};
use mine_data_strutcs::{
    curse::curse_modpacks::*,
    curse::curse_mods::*
};

use requester::{async_pool::AsyncPool, requester::request_maker::{CurseMethod, CurseRequester}};
use crate::zipper::pack_unzipper::unzip_temp_pack;
use mine_data_strutcs::url_maker::maker::Curse;

use reqwest::Response;
use super::functions::*;
use std::fs;

#[allow(unused)]
pub async fn curse_modpack_downloader(path: &str, destination_path: &str, mut n_threads: usize) {
    println!("{path}");
    unzip_temp_pack(path);

    if n_threads == 0 {
        n_threads = 64;
    }
    
    let curse_pack = load_curse_pack(&(TEMP_DIR.to_owned() + "manifest.json"))
        .expect("Couldnt load the pack");
    
    let files_ids: Vec<String>  = curse_pack.get_files()
        .iter()
        .map(|f|
            Curse::file(
                &f.get_projectID().to_string(), 
                &f.get_fileID().to_string()
            )
        )
        .collect(); 


    let curse_req = CurseRequester::new();
    
    // Get the info of each mod to get the url and download it 
    let responses: Vec<Response> = get_mod_responses(&curse_req, files_ids, n_threads).await;        
    let mut names = Vec::new();

    let mods_path = destination_path.to_owned() + "mods/";

    let download_urls = get_download_urls(&curse_req, responses, &mut names).await;
    let responses = download_mods(&curse_req, download_urls).await;
    let writters = get_writters(responses, names, &mods_path).await;
    let mut pool = AsyncPool::new();
    pool.push_request_vec(writters);
    pool.start().await;
    overrides(destination_path);
}


async fn get_mod_responses(
    curse_req: &CurseRequester,
    files_ids: Vec<String>,
    n_threads: usize
) -> Vec<Response> {
 
    let mut responses: Vec<Response> = Vec::with_capacity(files_ids.len());

    // Split the files ids into chunks so Uranium dont spawn
    // 5784923543 threads 
    for chunk in files_ids.chunks(n_threads){
        let mut requests = Vec::new();
        for url in chunk{
            let tarea = curse_req.get(url.clone(), CurseMethod::GET, "").await;
            requests.push(tarea); 
        }
        let mut pool = AsyncPool::new();
        pool.push_request_vec(requests);
        pool.start().await;
        
        // Wait for the current pool to end and append the results
        // to the results vector
        responses.append(
            &mut pool.get_done_request()
                .into_iter()
                .map(|f| 
                    match f {
                        Ok(val) => Some(val),
                        Err(e)  => {println!("{:?}", e); None}
                    }
                )
                .collect::<Vec<Option<Response>>>()
                .into_iter()
                .flatten()
                .collect()
        );
    }

    responses
}


#[allow(unused)]
async fn get_download_urls(
    curse_req: &CurseRequester, 
    responses: Vec<Response>, 
    names: &mut Vec<String>
) -> Vec<String>{
    let mut download_urls = Vec::new();
    for response in responses {
        let curse_file = response.json::<CurseResponse<CurseFile>>().await;
        if  curse_file.is_ok() {
            let curse_file = curse_file.unwrap();
            let download_url = curse_file.data.get_downloadUrl();            
            if download_url.is_empty(){
                println!("There is no download link for {}", curse_file.data.get_fileName());
            } else {
                names.push(curse_file.data.get_fileName());
                download_urls.push(download_url);
            }           
        } 
    }
    download_urls
}

async fn download_mods(
    curse_req: &CurseRequester, 
    download_urls: Vec<String>
) -> Vec<Response>{
    
    let chunks = download_urls.chunks(64).collect::<Vec<&[String]>>(); 
    let mut responses = Vec::new();


    for chunk in chunks {
        let urls_chunk = chunk.to_vec();

        let mut tareas = Vec::new();      
        let mut pool = AsyncPool::new();
        
        for download_url in urls_chunk {
            let tarea = curse_req.get(download_url, CurseMethod::GET, "").await;
            tareas.push(tarea);
        }
    
        pool.push_request_vec(tareas);
        pool.start().await;

        let mut chunk_responses: Vec<Response> = pool
            .get_done_request()
            .into_iter()
            .flatten()
            .collect();

        responses.append(&mut chunk_responses);
    }

    responses
}

fn overrides(destination_path: &str){

    // Copy all the content of overrides into the minecraft root folder

    let options = fs_extra::dir::CopyOptions::new();
    let file_options = fs_extra::file::CopyOptions::new();
    let overrides_folder = (TEMP_DIR.to_owned() + "overrides").to_owned();


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

