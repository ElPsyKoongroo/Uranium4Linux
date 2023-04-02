use super::functions::get_writters;
use crate::checker::{dlog, log, olog};
use crate::code_functions::N_THREADS;
use crate::downloaders::functions::overrides;
use crate::{
    variables::constants::{RINTH_JSON, TEMP_DIR},
    zipper::pack_unzipper::unzip_temp_pack,
};
use mine_data_strutcs::rinth::rinth_packs::{load_rinth_pack, RinthMdFiles, RinthModpack};
use requester::async_pool::AsyncPool;
use requester::mod_searcher::search_by_url;
use reqwest::Response;
use tokio::task::JoinSet;

// TODO!
// This should return an url with all the ids in the modpack, also
// the mods without the id so we can download them later one by one
fn build_url(rp: &RinthModpack) {
    rp.get_files()
        .iter()
        .for_each(|f| println!("{:?}", f.get_id()));
}

pub async fn download_rinth_pack(path: &str, destination_path: &str) {
    unzip_temp_pack(path);

    let rinth_pack = load_rinth_pack(&(TEMP_DIR.to_owned() + RINTH_JSON));

    dlog("Pack loaded");

    let file_links: Vec<&str> = rinth_pack
        .get_files()
        .iter()
        .map(RinthMdFiles::get_download_link_raw)
        .collect();

    dlog(format!("Downloading {} files", file_links.len()));

    let file_names: Vec<&str> = rinth_pack
        .get_files()
        .iter()
        .map(RinthMdFiles::get_raw_name)
        .collect();

    let responses = download_mods(&file_links).await;
    write_mods(responses, &file_names, destination_path).await;
    overrides(&destination_path.into(), "overrides");
}

async fn download_mods(links: &[&str]) -> Vec<Response> {
    let requester = reqwest::Client::new();
    let mut final_data = Vec::with_capacity(links.len());
    #[cfg(feature = "console_output")]
    let mut percent: f32 = 0.0;

    let mut join_set = JoinSet::new();
    for chunk in links.chunks(N_THREADS()) {

        for f in chunk {
            join_set.spawn(search_by_url(&requester, f));
        }
        
        while let Some(Ok(Ok(data))) = join_set.join_next().await {
            match data {
                Ok(e) => final_data.push(e),
                Err(err) => log(format!("Error {}", err)),
            }
        }
        #[cfg(feature = "console_output")]
        {
            percent += chunk.len() as f32 / links.len() as f32 * 100.0;
            println!("{:0.2} %", percent);
        }
    }
    final_data
}

#[allow(unused)]
async fn download_memory_perf(links: &[&str], names: Vec<&str>, destination_path: &str) {
    let requester = reqwest::Client::new();

    let mut pool = AsyncPool::new();
    for (url_chunk, name_chunk) in names.chunks(N_THREADS()).zip(names.chunks(N_THREADS())) {
        let mut tasks = Vec::with_capacity(url_chunk.len());

        url_chunk
            .iter()
            .for_each(|f| tasks.push(search_by_url(&requester, f)));

        pool.push_request_vec(tasks);

        let done_requests = pool.start().await.into_iter().flatten().collect();
        write_mods(done_requests, name_chunk, destination_path).await;
        pool.clear();
    }
}

async fn write_mods(responses: Vec<Response>, names: &[&str], destination_path: &str) {
    let mut join_set = JoinSet::new();
    let mods_path = destination_path.to_owned() + "mods/";
    get_writters(responses, names, &mods_path)
        .await
        .into_iter()
        .for_each(|j| {join_set.spawn(j);});

    while let Some(Ok(res)) = join_set.join_next().await {
        match res {
            Ok(_) => {},
            Err(e) => log(e),
        }
    }

    /*
    let mut pool = AsyncPool::new();
    pool.push_request_vec(writters);
    pool.start().await;
    */
}
