use mine_data_strutcs::rinth::rinth_mods::*;
use mine_data_strutcs::url_maker::maker;
use serde::{de::DeserializeOwned, Serialize};

pub enum SEARCH_TYPE {
    QUERRY(String),
    FOR(u32, u32),
    MOD(String),
    PROJECT(String),
    VERSION(String),
    VERSIONS(String),
    RESOURCEPACKS(u32, u32),
}

pub async fn search(search: SEARCH_TYPE) {
    match search {
        SEARCH_TYPE::QUERRY(_) => {todo!()}
        SEARCH_TYPE::FOR(limit, offset) => search_for(limit, offset).await,
        SEARCH_TYPE::MOD(_) => {todo!()}
        SEARCH_TYPE::PROJECT(_) => {todo!()}
        SEARCH_TYPE::VERSION(id) => search_version(&id).await,
        SEARCH_TYPE::VERSIONS(_) => {todo!()}
        SEARCH_TYPE::RESOURCEPACKS(limit, offset) => search_sourcepacks(limit, offset).await,
    }
}

#[allow(unused)]
async fn get(id: &str) {
    let url = maker::ModRinth::mod_version_by_id(id);
    let version = get_data::<RinthVersion>(&url).await;
    let data = get_data::<Vec<u8>>(&version.get_file_url()).await;
    write_file(data).await;
}

async fn search_sourcepacks(limit: u32, offset: u32) {
    let url = maker::ModRinth::resourcepacks(limit, offset);
    let data = get_data::<RinthResponse>(&url).await;
    write_data(data).await;
}

async fn search_version(id: &str) {
    let url = maker::ModRinth::mod_version_by_id(id);
    let data = get_data::<RinthVersion>(&url).await;
    write_data(data).await;
}

async fn search_for(limit: u32, offset: u32) {
    let url = maker::ModRinth::search_for(limit, offset);
    let data = get_data::<RinthResponse>(&url).await;
    write_data(data).await;
}

async fn get_data<T: DeserializeOwned>(url: &str) -> T {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.unwrap();
    response.json::<T>().await.unwrap()
}

async fn write_data<T: Serialize>(data: T) {
    let bytes = serde_json::to_vec(&data).unwrap();
    let coso = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", coso);
    tokio::fs::write("response.json", bytes).await.unwrap();
}

async fn write_file(data: Vec<u8>) {
    tokio::fs::write("mod.jar", data).await.unwrap();
}
