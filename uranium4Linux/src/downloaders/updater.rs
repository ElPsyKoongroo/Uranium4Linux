use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use mine_data_strutcs::{rinth::rinth_mods::RinthVersion, url_maker};
use requester::mod_searcher::search_by_url_post;

use crate::hashes::rinth_hash;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Content {
    hashes: Vec<String>,
    algorithm: String,
    loaders: Vec<String>,
    game_versions: Vec<String>,
}

impl Content {
    pub fn new(hashes: Vec<String>, game_versions: Vec<String>) -> Content {
        Content {
            hashes,
            algorithm: "sha1".to_owned(),
            loaders: vec!["fabric".to_owned()],
            game_versions,
        }
    }
}

pub async fn update_modpack(minecraft_path: &str) {
    let mods_path = minecraft_path.to_owned() + "mods/";
    let mods_names = std::fs::read_dir(&mods_path).unwrap();
    let mods_hashes = mods_names
        .map(|f| rinth_hash(f.unwrap().path().to_str().unwrap()))
        .collect::<Vec<String>>();

    let updates = get_updates(&mods_hashes).await;

    for hash in mods_hashes {
        match updates.get(&hash) {
            Some(v) if v.get_hashes().sha1 != hash => {
                println!("Update avaliable for {}", v.get_name());
            }
            Some(v) => {
                println!("{} is up to date!", v.get_name());
            }
            None => {}
        }
    }

    // TODO update!
}

async fn get_updates(mods_hashes: &[String]) -> HashMap<String, RinthVersion> {
    let cliente = reqwest::Client::new();
    let post_content = Content::new(mods_hashes.to_owned(), vec!["1.19.2".to_owned()]);
    let url = url_maker::maker::ModRinth::update_by_hash_post();
    let response = search_by_url_post(&cliente, &url, &post_content)
        .await
        .unwrap()
        .unwrap();

    response
        .json::<HashMap<String, RinthVersion>>()
        .await
        .unwrap()
}
