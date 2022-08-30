use super::functions::get_writters;
use crate::{variables::constants::{TEMP_DIR, RINTH_JSON_NAME}, zipper::pack_unzipper::unzip_temp_pack};
use mine_data_strutcs::rinth::rinth_packs::*;
use requester::async_pool::AsyncPool;
use reqwest::Response;

pub async fn download_rinth_pack(path: &str, destination_path: &str, n_threads: usize) {
    unzip_temp_pack(path);

    let rinth_pack = load_rinth_pack(&(TEMP_DIR.to_owned() + RINTH_JSON_NAME)).unwrap();

    let file_links: Vec<String> = rinth_pack
        .get_files()
        .iter()
        .map(|f| f.get_download_link())
        .collect();

    let file_names: Vec<String> = rinth_pack
        .get_files()
        .iter()
        .map(|f| f.get_name())
        .collect();

    let responses = download_mods(file_links, n_threads).await;
    write_mods(responses, file_names, destination_path).await;
}

async fn download_mods(links: Vec<String>, n_threads: usize) -> Vec<Response> {
    let requester = reqwest::Client::new();

    let chunks = links.chunks(n_threads).collect::<Vec<&[String]>>();
    let mut final_data = Vec::with_capacity(links.len());
    let mut percent: f32 = 0.0;
    for chunk in chunks {
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

async fn write_mods(responses: Vec<Response>, names: Vec<String>, destination_path: &str) {
    let writters = get_writters(responses, names, &(destination_path.to_owned() + "mods/")).await;
    let mut pool = AsyncPool::new();
    pool.push_request_vec(writters);
    pool.start().await;
}
