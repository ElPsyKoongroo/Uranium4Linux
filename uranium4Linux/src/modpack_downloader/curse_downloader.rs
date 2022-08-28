use crate::variables::constants::TEMP_DIR;
use crate::zipper::pack_unzipper::unzip_temp_pack;
use mine_data_strutcs::url_maker::maker::Curse;
use mine_data_strutcs::{curse::curse_modpacks::*, curse::curse_mods::*};
use requester::{
    async_pool::AsyncPool, mod_searcher::Method, requester::request_maker::CurseRequester,
};

use super::functions::*;
use reqwest::Response;

pub async fn curse_modpack_downloader(path: &str, destination_path: &str, n_threads: usize) {
    println!("{path}");
    unzip_temp_pack(path);

    let curse_pack =
        load_curse_pack(&(TEMP_DIR.to_owned() + "manifest.json")).expect("Couldnt load the pack");

    let files_ids: Vec<String> = curse_pack
        .get_files()
        .iter()
        .map(|f| Curse::file(&f.get_projectID().to_string(), &f.get_fileID().to_string()))
        .collect();

    let curse_req = CurseRequester::new();

    // Get the info of each mod to get the url and download it
    let responses: Vec<Response> = get_mod_responses(&curse_req, files_ids, n_threads).await;
    let mut names = Vec::new();
  
    let mods_path = destination_path.to_owned() + "mods/";

    let download_urls = get_download_urls(&curse_req, responses, &mut names).await;
    let responses = download_mods(&curse_req, download_urls, &names, &mods_path, n_threads).await;
    let writters = get_writters(responses, names, &mods_path).await;
    let mut pool = AsyncPool::new();
    pool.push_request_vec(writters);
    pool.start().await;
    overrides(destination_path, "overrides");
}

async fn get_mod_responses(
    curse_req: &CurseRequester,
    files_ids: Vec<String>,
    n_threads: usize,
) -> Vec<Response> {
    let mut responses: Vec<Response> = Vec::with_capacity(files_ids.len());

    // Split the files ids into chunks so Uranium dont spawn
    // 5784923543 threads
    for chunk in files_ids.chunks(n_threads) {
        let mut requests = Vec::new();
        for url in chunk {
            let tarea = curse_req.get(url, Method::GET, "");
            requests.push(tarea);
        }
        let mut pool = AsyncPool::new();
        pool.push_request_vec(requests);
        pool.start().await;

        // Wait for the current pool to end and append the results
        // to the results vector
        responses.append(
            &mut pool
                .get_done_request()
                .into_iter()
                .map(|f| match f {
                    Ok(val) => Some(val),
                    Err(e) => {
                        println!("{:?}", e);
                        None
                    }
                })
                .flatten()
                .collect(),
        );
    }

    responses
}

#[allow(unused)]
async fn get_download_urls(
    curse_req: &CurseRequester,
    responses: Vec<Response>,
    names: &mut Vec<String>,
) -> Vec<String> {
    // In order to get rid of reallocations pre allocate the vector with
    // responses capacity.
    // The vector rarelly will get full beacause of empty links.
    let mut download_urls = Vec::with_capacity(responses.len());

    for response in responses {
        // Parse the response into a CurseResponse<CurseFile>
        let curse_file = response.json::<CurseResponse<CurseFile>>().await;

        // If everything ok
        if curse_file.is_ok() {
            let curse_file = curse_file.unwrap();
            let download_url = curse_file.data.get_downloadUrl();

            // In case the download link its empty, because CurseApi could give
            // a right response but with empty download link... -.-
            if download_url.is_empty() {
                println!(
                    "There is no download link for {}",
                    curse_file.data.get_fileName()
                );
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
    download_urls: Vec<String>,
    names: &Vec<String>,
    mods_path: &str,
    n_threads: usize,
) -> Vec<Response> {
    let chunks = download_urls.chunks(n_threads).collect::<Vec<&[String]>>();
    let names_chunks = names.chunks(n_threads).collect::<Vec<&[String]>>();
    let mut responses = Vec::with_capacity(download_urls.len());

    // Get all the files in chunks of n_threads elements
    for (chunk, names_c) in chunks.iter().zip(names_chunks.iter()) {
        let mut tareas = Vec::with_capacity(chunk.len());
        let mut pool = AsyncPool::new();

        // Add the tasks for this chunk
        for download_url in *chunk {
            let tarea = curse_req.get(download_url, Method::GET, "");
            tareas.push(tarea);
        }
        pool.push_request_vec(tareas);
        pool.start().await;

        // Collect the responses and then push them into responses vector
        let mut chunk_responses: Vec<Response> =
            pool.get_done_request().into_iter().flatten().collect();

        // Experimental 
        /*let writters = get_writters(chunk_responses, names_c.to_vec(), &mods_path).await;
        let mut pool = AsyncPool::new();
        pool.push_request_vec(writters);
        pool.start().await;        */

        responses.append(&mut chunk_responses);
    }
    responses
}
