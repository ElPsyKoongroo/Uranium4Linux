use crate::{
    code_functions::N_THREADS,
    variables::constants::{CURSE_JSON, TEMP_DIR},
    zipper::pack_unzipper::unzip_temp_pack,
};
use mine_data_strutcs::{
    curse::{curse_modpacks::*, curse_mods::*},
    url_maker::maker::Curse,
};
use requester::{
    async_pool::AsyncPool,
    mod_searcher::Method,
    requester::request_maker::{CurseRequester, Req},
};
use reqwest::Response;
use std::{path::PathBuf, sync::Arc};

use super::functions::overrides;
use super::gen_downloader::Downloader;

pub async fn curse_modpack_downloader(path: &str, destination_path: &str) {
    unzip_temp_pack(path);

    let curse_pack = load_curse_pack((TEMP_DIR.to_owned() + CURSE_JSON).as_ref())
        .expect("Couldnt load the pack");

    let files_ids: Vec<String> = curse_pack
        .get_files()
        .iter()
        .map(|f| Curse::file(&f.get_projectID().to_string(), &f.get_fileID().to_string()))
        .collect();

    let curse_req = CurseRequester::new();

    // Get the info of each mod to get the url and download it
    let responses: Vec<Response> = get_mod_responses(&curse_req, &files_ids).await;
    let mut names = Vec::with_capacity(files_ids.len());

    let mods_path = destination_path.to_owned() + "mods/";

    let download_urls = get_download_urls(&curse_req, responses, &mut names).await;

    let names: Vec<PathBuf> = names.iter().map(|n| PathBuf::from(n)).collect();

    let downloader = Downloader {
        names,
        urls: Arc::new(download_urls),
        path: Arc::new(PathBuf::from(mods_path)),
        requester: curse_req,
    };
    downloader.start().await;

    overrides(&destination_path.into(), "overrides");
}

async fn get_mod_responses(curse_req: &CurseRequester, files_ids: &[String]) -> Vec<Response> {
    let mut responses: Vec<Response> = Vec::with_capacity(files_ids.len());
    let threads: usize = N_THREADS();

    for chunk in files_ids.chunks(threads) {
        let mut pool = AsyncPool::new();
        let mut requests = Vec::new();
        for url in chunk {
            let tarea = curse_req.get(url, Method::GET, "");
            requests.push(tarea);
        }
        pool.push_request_vec(requests);

        // Wait for the current pool to end and append the results
        // to the results vector
        responses.append(
            &mut pool
                .start()
                .await
                .into_iter()
                .filter_map(|f| match f {
                    Ok(val) => Some(val),
                    Err(e) => {
                        println!("{:?}", e);
                        None
                    }
                })
                .collect(),
        );
        // pool.clear();
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
        if let Ok(file) = curse_file {
            let download_url = file.data.get_downloadUrl();

            // In case the download link its empty, because CurseApi could give
            // a right response but with empty download link... -.-
            if download_url.is_empty() {
                println!("There is no download link for {}", file.data.get_fileName());
            } else {
                names.push(file.data.get_fileName());
                download_urls.push(download_url);
            }
        }
    }
    download_urls
}
