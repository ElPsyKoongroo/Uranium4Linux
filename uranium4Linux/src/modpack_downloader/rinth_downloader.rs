use super::functions::get_writters;
use crate::code_functions::N_THREADS;
use crate::{
    variables::constants::{RINTH_JSON_NAME, TEMP_DIR},
    zipper::pack_unzipper::unzip_temp_pack,
};
use mine_data_strutcs::rinth::rinth_packs::*;
use requester::async_pool::AsyncPool;
use reqwest::Response;

pub async fn download_rinth_pack(path: &str, destination_path: &str) {
    unzip_temp_pack(path);

    let rinth_pack = load_rinth_pack(&(TEMP_DIR.to_owned() + RINTH_JSON_NAME));

    let file_links: Vec<String> = rinth_pack
        .get_files()
        .iter()
        .map(RinthMdFiles::get_download_link)
        .collect();

    let file_names: Vec<String> = rinth_pack
        .get_files()
        .iter()
        .map(RinthMdFiles::get_name)
        .collect();

    let responses = download_mods(file_links).await;
    write_mods(responses, file_names, destination_path).await;
}

async fn download_mods(links: Vec<String>) -> Vec<Response> {
    let requester = reqwest::Client::new();

    let mut final_data = Vec::with_capacity(links.len());

    #[cfg(feature = "console_output")]
    let mut percent: f32 = 0.0;

    for chunk in links.chunks(N_THREADS()) {
        let mut pool = AsyncPool::new();
        let mut tasks = Vec::with_capacity(chunk.len());

        chunk
            .iter()
            .for_each(|f| tasks.push(tokio::task::spawn(requester.get(f).send())));

        pool.push_request_vec(tasks);
        pool.start().await;

        #[cfg(feature = "console_output")]
        {
            percent += chunk.len() as f32 / links.len() as f32 * 100.0;
            println!("{:0.2}%", percent);
        }

        final_data.append(&mut pool.get_done_request());
    }

    final_data.into_iter().flatten().collect()
}

#[allow(unused)]
async fn download_memory_perf(links: Vec<String>, names: Vec<String>, destination_path: &str) {
    let requester = reqwest::Client::new();

    for (url_chunk, name_chunk) in names.chunks(N_THREADS()).zip(names.chunks(N_THREADS())) {
        let mut pool = AsyncPool::new();
        let mut tasks = Vec::with_capacity(url_chunk.len());

        url_chunk
            .iter()
            .for_each(|f| tasks.push(tokio::task::spawn(requester.get(f).send())));

        pool.push_request_vec(tasks);
        pool.start().await;

        let done_requests = pool.get_done_request().into_iter().flatten().collect();

        write_mods(done_requests, name_chunk.to_vec(), destination_path).await;
    }
}

async fn write_mods(responses: Vec<Response>, names: Vec<String>, destination_path: &str) {
    let mods_path = destination_path.to_owned() + "mods/";
    let writters = get_writters(responses, names, &mods_path).await;
    let mut pool = AsyncPool::new();
    pool.push_request_vec(writters);
    pool.start().await;
}
