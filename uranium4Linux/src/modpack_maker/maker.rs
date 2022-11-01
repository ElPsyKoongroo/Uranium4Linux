use crate::hashes::rinth_hash;
use crate::variables::constants;
use crate::{easy_input, zipper::pack_zipper::compress_pack};
use core::panic;
use futures::future::join_all;
use mine_data_strutcs::{
    rinth::rinth_mods::RinthVersion, rinth::rinth_packs::RinthModpack, url_maker::maker,
};
use requester::async_pool::AsyncPool;
use std::fs::read_dir;
use std::path::Path;

enum ParseState {
    Good(RinthVersion),
    Raw(String),
}


#[cfg(not(feature = "console_input"))]
pub async fn make_modpack(path: &str, n_threads: usize) {

    let hash_filename = get_mods(Path::new(path));

    let mods_states = search_mods_for_modpack(hash_filename, n_threads).await;

    let mp_name = easy_input::input("Modpack name: ", String::from("Modpack"));

    let mut json_name = mp_name.clone();
    fix_name(&mut json_name);

    //uranium_pack.write_mod_pack_with_name(&json_name)

    let mut rinth_pack = RinthModpack::new();
    let mut raw_mods = Vec::new();
    for rinth_mod in mods_states {
        match rinth_mod {
            ParseState::Good(m) => rinth_pack.add_mod(m.into()),
            ParseState::Raw(file_name) => raw_mods.push(file_name),
        }
    };

    rinth_pack.write_mod_pack_with_name();

    compress_pack(&mp_name, path, &raw_mods).unwrap();

    std::fs::remove_file(constants::RINTH_JSON).unwrap();
}

fn get_mods(minecraft_path: &Path) -> Vec<(String, String)> {
    let mut hashes_names = Vec::new();
    assert!(minecraft_path.is_dir(), "{:?} is not a dir", minecraft_path);

    let mods_path = minecraft_path.join("mods/");

    let mods = match read_dir(&mods_path) {
        Ok(e) => e
            .into_iter()
            .map(|f| f.unwrap().path().to_str().unwrap().to_owned())
            .collect::<Vec<String>>(),
        Err(error) => {
            eprintln!("Error reading the directore: {}", error);
            panic!("")
        }
    };

    // Push all the (has, file_name) to the vector
    for path in mods {
        let rinth = rinth_hash(&path);
        let file_name = path.split('/').last().unwrap().to_owned();
        hashes_names.push((rinth, file_name));
    }

    hashes_names
}

/// Search the mods in mods/ in `RinthAPI` by hash,
/// if cant find it, add it to `not_found_mods` and later
/// add them raw to the modpack.
async fn search_mods_for_modpack(
    hash_filename: Vec<(String, String)>,
    n_threads: usize,
) -> Vec<ParseState> {
    search_mod(&hash_filename, n_threads).await
}

/// Returns a tuple of Vectos, the first one with the mods found in Rinth Repo
/// and the second one the names of the raw mods
async fn search_mod(item: &[(String, String)], n_threads: usize) -> Vec<ParseState> {
    let n_mods = item.len();
    let cliente = reqwest::Client::new();
    let chunks = item
        .chunks(n_threads)
        .collect::<Vec<&[(String, String)]>>();

    // Get rinth_responses
    let mut rinth_responses = Vec::with_capacity(n_mods);
    for chunk in &chunks {
        let mut pool = AsyncPool::new();
        let reqs = chunk
            .iter()
            .map(|f| tokio::task::spawn(cliente.get(maker::ModRinth::hash(&f.0)).send()))
            .collect();
        pool.push_request_vec(reqs);
        pool.start().await;
        rinth_responses.append(&mut pool.get_done_request());
    }

    // Get rinth parses
    let rinth_parses = join_all(
        rinth_responses
            .into_iter()
            .map(|request| request.unwrap().json::<RinthVersion>()),
    )
    .await;

    let mut mods_states = Vec::with_capacity(n_mods);

    for (i, rinth) in rinth_parses.into_iter().enumerate() {
        if let Ok(m) = rinth {
            mods_states.push(ParseState::Good(m));
        } else {
            mods_states.push(ParseState::Raw(item[i].1.clone()));
        }
    }
    mods_states
}

fn get_curse_body(id: &str) -> String {
    format!(
        "{{
            \"fingerprints\": [
                {}
            ]
       }}",
        id
    )
}

fn fix_name(name: &mut String) {
    if name.ends_with(".json") {
        name.strip_suffix(".json").unwrap();
    }
    name.push_str("_temp.json");
}
