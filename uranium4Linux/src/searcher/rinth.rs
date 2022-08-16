use mine_data_strutcs::url_maker::maker;
use mine_data_strutcs::rinth::rinth_mods::*;
use serde::{Serialize, de::DeserializeOwned};

pub enum SEARCH_TYPE {
    QUERRY(String),
    FOR(u32, u32),
    MOD(String),
    PROJECT(String),
    VERSION(String),
    VERSIONS(String)
}


pub async fn search(search: SEARCH_TYPE) {
    match search {
        SEARCH_TYPE::QUERRY(_) => {},
        SEARCH_TYPE::FOR(limit, offset) => {search_for(limit, offset).await},
        SEARCH_TYPE::MOD(_) => {},
        SEARCH_TYPE::PROJECT(_) => {},
        SEARCH_TYPE::VERSION(id) => {search_version(&id).await},
        SEARCH_TYPE::VERSIONS(_) => {}
    }
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
    let data = response.json::<T>().await.unwrap();
    data
}

async fn write_data<T: Serialize>(data: T) {
    let bytes = serde_json::to_vec(&data).unwrap();
    tokio::fs::write("response.json", bytes).await.unwrap();
}
