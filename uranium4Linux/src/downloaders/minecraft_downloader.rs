use mine_data_strutcs::minecraft;
use requester::{async_pool::AsyncPool, mod_searcher::search_by_url};
//use super::functions::get_writters;
use reqwest;

use crate::code_functions::N_THREADS;

const ASSESTS_PATH: &str = "assets/";
const PLACE_HOLDER: &str =
    "https://piston-meta.mojang.com/v1/packages/c492375ded5da34b646b8c5c0842a0028bc69cec/2.json";

pub async fn donwload_minecraft(destionation_path: &str) -> Result<(), reqwest::Error> {
    std::fs::create_dir("assets/indexes");
    std::fs::create_dir("assets/objects");

    let requester = reqwest::Client::new();
    let resources = requester
        .get(PLACE_HOLDER)
        .send()
        .await?
        .json::<minecraft::Resources>()
        .await?;
    download_resources(destionation_path, resources, &requester).await;
    Ok(())
}

pub async fn download_resources(
    destionation_path: &str,
    resources: minecraft::Resources,
    requester: &reqwest::Client,
) {
    let (names_vec, data): (Vec<String>, Vec<minecraft::ObjectData>) = resources
        .objects
        .files
        .into_iter()
        .map(|(_, b)| (b.hash.clone().unwrap_or_default(), b))
        .unzip();
    
    let chunk_size = N_THREADS();

    let mut responses = Vec::new();
    let mut i = 0;

    for files in data.chunks(chunk_size) {
        let mut pool = AsyncPool::new();
        let mut tasks = Vec::with_capacity(chunk_size);

        files
            .iter()
            .for_each(|file| tasks.push(search_by_url(requester, &file.get_link())));

        pool.push_request_vec(tasks);

        pool.start().await;
        responses.push(pool.get_done_request().into_iter().flatten().collect::<Vec<reqwest::Response>>());
        println!("{i}");
    }

    dbg!("Requests terminadas; Empezando escritura");
    i = 0;
    for (chunk, names) in responses.into_iter().zip(names_vec.chunks(chunk_size)) {
        write_chunk(get_writters(chunk, names).await).await;
        println!("{i}");
    }
}

pub async fn write_chunk(writters: Vec<tokio::task::JoinHandle<()>>) {
    let mut pool = AsyncPool::new();
    pool.push_request_vec(writters);
    pool.start().await
}

/*
 *
 *
 * */

async fn get_writters(data: Vec<reqwest::Response>, names: &[String]) -> Vec<tokio::task::JoinHandle<()>>{
    let mut writters = Vec::with_capacity(data.len());
    for (data, name) in data.into_iter().zip(names) {
        writters.push(
            tokio::task::spawn(write_mod(data, name.clone()))
        );
    }
    writters
}

async fn write_mod(data: reqwest::Response, name: String) {
    let path = ASSESTS_PATH.to_owned() + "objects/" + &name[..2] + "/" + &name;
    let content = data.bytes().await.unwrap_or_default();
    match std::fs::write(&path, &content) {
        Ok(_) => {}
        Err(_e) => {
            std::fs::create_dir_all(ASSESTS_PATH.to_owned() + "objects/" + &name[..2] + "/").expect("No se pudo crear el directorio");
            std::fs::write(path, &content).expect("No se pudo escribir el objeto");
        }
    }
}
