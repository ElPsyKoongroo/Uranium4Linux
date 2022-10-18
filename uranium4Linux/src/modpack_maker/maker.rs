use crate::hashes::{curse_hash, rinth_hash};
use crate::{checker::check, easy_input, zipper::pack_zipper::compress_pack};
use mine_data_strutcs::uranium_modpack::modpack_mod::Mods;
use mine_data_strutcs::uranium_modpack::modpack_struct::UraniumPack;
use mine_data_strutcs::{
    curse::curse_mods::{CurseFingerPrint, CurseResponse},
    rinth::rinth_mods::RinthVersion,
    url_maker::maker,
};
use requester::async_pool::AsyncPool;
use requester::mod_searcher::Method;
use requester::requester::request_maker::CurseRequester;
use core::panic;
use std::fs::read_dir;
use std::path::Path;

struct ModHashes {
    pub rinth_hash: String,
    pub curse_hash: String,
}

pub struct ModAttributes {
    pub name: String,
    pub author: String,
    pub version: String,
}

#[cfg(feature = "console_input")]
pub async fn make_modpack(path: &str, n_threads: usize, attr: ModAttributes) {
    let hash_filename = get_mods(Path::new(path)).unwrap();

    let mut uranium_pack = search_mods_for_modpack(hash_filename, n_threads).await;

    let mp_name = easy_input::input("Modpack name: ", String::from("Modpack"));
    let mp_version = easy_input::input("Modpack version: ", String::from("1.0"));
    let mp_author = easy_input::input("Modpack author: ", String::from("Anonimous"));

    let mut json_name = mp_name.clone();
    fix_name(&mut json_name);

    uranium_pack.set_name(attr.name);
    uranium_pack.set_version(attr.version);
    uranium_pack.set_author(attr.author);

    uranium_pack.write_mod_pack_with_name(&json_name);

    compress_pack(&mp_name, path, &Vec::new() /*not_found_mods*/).unwrap();

    std::fs::remove_file(json_name).unwrap();
}

#[cfg(not(feature = "console_input"))]
pub async fn make_modpack(path: &str, n_threads: usize) {
    let hash_filename = get_mods(Path::new(path));

    let (mut uranium_pack, raw_mods) = search_mods_for_modpack(hash_filename, n_threads).await;

    let mp_name = easy_input::input("Modpack name: ", String::from("Modpack"));
    let mp_version = easy_input::input("Modpack version: ", String::from("1.0"));
    let mp_author = easy_input::input("Modpack author: ", String::from("Anonimous"));

    let mut json_name = mp_name.clone();
    fix_name(&mut json_name);

    uranium_pack.set_name(mp_name.clone());
    uranium_pack.set_version(mp_version);
    uranium_pack.set_author(mp_author);

    uranium_pack.write_mod_pack_with_name(&json_name);

    compress_pack(&mp_name, path, &raw_mods /*not_found_mods*/).unwrap();

    std::fs::remove_file(json_name).unwrap();
}

fn get_mods(minecraft_path: &Path) -> Vec<(ModHashes, String)> {
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
        let curse = curse_hash(&path);
        let hashes = ModHashes {
            rinth_hash: rinth,
            curse_hash: curse,
        };
        let file_name = path.split('/').last().unwrap().to_owned();
        hashes_names.push((hashes, file_name));
    }

    hashes_names
}

/// Search the mods in mods/ in `RinthAPI` by hash,
/// if cant find it, add it to `not_found_mods` and later
/// add them raw to the modpack.
async fn search_mods_for_modpack(
    hash_filename: Vec<(ModHashes, String)>,
    n_threads: usize,
) -> (UraniumPack, Vec<String>) {
    let (mut mods, raw) = search_mod(&hash_filename, n_threads).await;
    let mut uranium_pack = UraniumPack::new();
    uranium_pack.append_mods(&mut mods);
    (uranium_pack, raw)
}

/// Returns a tuple of Vectos, the first one with the mods found in Rinth Repo
/// and the second one the names of the raw mods 
async fn search_mod(item: &[(ModHashes, String)], n_threads: usize) -> (Vec<Mods>, Vec<String>) {
    let n_mods = item.len();

    let cliente = reqwest::Client::new();

    let chunks = item
        .chunks(n_threads)
        .collect::<Vec<&[(ModHashes, String)]>>();


    // Get rinth_responses
    let mut rinth_responses = Vec::new();
    for chunk in &chunks {
        let mut pool = AsyncPool::new();
        let reqs = chunk
            .iter()
            .map(|f| tokio::task::spawn(cliente.get(maker::ModRinth::hash(&f.0.rinth_hash)).send()))
            .collect();
        pool.push_request_vec(reqs);
        pool.start().await;
        rinth_responses.append(&mut pool.get_done_request());
    }

    // Get rinth parses
    let mut rinth_parses = Vec::with_capacity(n_mods);
    for response in rinth_responses {
        rinth_parses.push(check(
            response.unwrap().json::<RinthVersion>().await,
            false,
            false,
            "",
        ));
    }

    let mut mods_data = Vec::with_capacity(n_mods);
    let mut raw_mods = Vec::new();

    for (i, rinth) in rinth_parses.into_iter().enumerate() {
        match rinth {
            Some(m) => { 
                println!("{} OK", item[i].1);
                mods_data.push(Mods::from_RinthVersion(&m))
            },
            None => {
                println!("{} OK", item[i].1);
                raw_mods.push(item[i].1.clone())
            }
        }
    }

    (mods_data,raw_mods)
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
