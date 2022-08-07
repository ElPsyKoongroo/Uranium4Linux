use crate::code_functions::into_chunks;
use crate::hashes::{curse_hash, rinth_hash};
use crate::{checker::check, easy_input, zipper::pack_zipper::compress_pack};
use mine_data_strutcs::{
    curse::curse_mods::{CurseFingerPrint, CurseResponse},
    rinth::rinth_mods::{RinthVersion, RinthVersions},
    url_maker::maker,
};
use requester::async_pool::AsyncPool;
use requester::mod_searcher::Method;
use requester::requester::request_maker::CurseRequester;
use std::fs::read_dir;
use std::path::Path;
use crate::variables::constants::N_THREADS;

struct ModHashes {
    pub rinth_hash: String,
    pub curse_hash: String,
}

pub async fn make_modpack(path: &str, n_threads: usize) {
    let hash_filename = get_mods(Path::new(path)).unwrap();

    let mut responses: RinthVersions = RinthVersions::new();
    let mut not_found_mods: Vec<String> = Vec::new();
    search_mods_for_modpack(hash_filename, &mut responses, &mut not_found_mods, n_threads).await;

    let mp_name = easy_input::input("Modpack name: ", String::from("Modpack"));
    let mp_version = easy_input::input("Modpack version: ", String::from("1.0"));
    let mp_author = easy_input::input("Modpack author: ", String::from("Anonimous"));

    let mut json_name = mp_name.clone();
    fix_name(&mut json_name);

    let mp = mine_data_strutcs::uranium_modpack::modpack_struct::ModPack::modpack_from_RinthVers(
        &mp_name, mp_version, mp_author, responses,
    );

    mp.write_mod_pack_with_name(&json_name);

    compress_pack(&mp_name, path, not_found_mods).unwrap();

    std::fs::remove_file(json_name).unwrap();
}

fn get_mods(minecraft_path: &Path) -> Option<Vec<(ModHashes, String)>> {
    let mut hashes_names = Vec::new();
    let mods;

    if !minecraft_path.is_dir() {
        return None;
    }
    let mods_path = minecraft_path.join("mods/");

    match read_dir(&mods_path) {
        Ok(e) => {
            mods = e
                .into_iter()
                .map(|f| f.unwrap().path().to_str().unwrap().to_owned())
                .collect::<Vec<String>>()
        }
        Err(error) => {
            eprintln!("Error reading the directore: {}", error);
            return None;
        }
    }

    // Push all the (has, file_name) to the vector
    for path in mods {
        let rinth = rinth_hash(&path);
        let curse = curse_hash(&path);
        let hashes = ModHashes {
            rinth_hash: rinth,
            curse_hash: curse,
        };
        let file_name = path.split("/").last().unwrap().to_owned();
        hashes_names.push((hashes, file_name));
    }

    Some(hashes_names)
}

/// Search the mods in mods/ in RinthAPI by hash,
/// if cant find it, add it to not_found_mods and later
/// add them raw to the modpack.
async fn search_mods_for_modpack(
    hash_filename: Vec<(ModHashes, String)>,
    responses: &mut RinthVersions,
    not_found_mods: &mut Vec<String>,
    n_threads: usize
) {
    let mut mods = search_mod(&hash_filename, n_threads).await;

    // Add the mod to responses if Ok, else add the file name to
    // not found mods for later add it as raw mod
    for i in 0..mods.len() {
        match mods.pop().unwrap() {
            Some(e) => responses.push(e),
            None => {
                println!("Not found {} !", hash_filename[i].1.to_owned());
                not_found_mods.push(hash_filename[i].1.to_owned())
            }
        }
    }
}

async fn search_mod(item: &Vec<(ModHashes, String)>, n_threads: usize) -> Vec<Option<RinthVersion>> {
    let n_mods = item.len();

    let curse_requester = CurseRequester::new();
    let cliente = reqwest::Client::new();

    let chunks = item.chunks(n_threads).collect::<Vec<&[(ModHashes, String)]>>();

    // Get curse responses by chunks
    let mut curse_responses = Vec::new();
    for chunk in chunks.iter() {
        let mut pool = AsyncPool::new();
        let reqs = chunk
            .iter()
            .map(|f| {
                curse_requester.get(
                    &maker::Curse::hash(),
                    Method::POST,
                    &get_curse_body(&f.0.curse_hash),
                )
            })
            .collect();
        pool.push_request_vec(reqs);
        pool.start().await;
        curse_responses.append(&mut pool.get_done_request());
    }

    // Get rinth_responses
    let mut rinth_responses = Vec::new();
    for chunk in chunks.iter() {
        let mut pool = AsyncPool::new();
        let reqs = chunk
            .iter()
            .map(|f| tokio::task::spawn(cliente.get(maker::ModRinth::hash(&f.0.rinth_hash)).send()))
            .collect();
        pool.push_request_vec(reqs);
        pool.start().await;
        rinth_responses.append(&mut pool.get_done_request());
    }

    // Get curse parses
    let mut curse_parses = Vec::with_capacity(n_mods);
    for response in curse_responses {
        curse_parses.push(check(
            response
                .unwrap()
                .json::<CurseResponse<CurseFingerPrint>>()
                .await,
            false,
            false,
            "",
        ));
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

    // Get the each mod info or None in
    // case neither Rinth or Curse has the mod
    let mut final_data = Vec::with_capacity(n_mods);
    for (rinth, curse) in rinth_parses.into_iter().zip(curse_parses.into_iter()) {
        // First try to add from Rinth
        if rinth.is_some() {
            final_data.push(rinth)
        }
        // If Rinth isnt avaliable, try with curse
        else {
            if curse.is_some() {
                let rinth_parse =
                    RinthVersion::from_CurseFile(curse.unwrap().data.get_file().clone());
                //We need to check if the url is empty bcs sometimes the curse
                //api returns empty download links
                if rinth_parse.get_file_url() != "" {
                    final_data.push(Some(rinth_parse));
                } else {
                    final_data.push(None);
                }
            }
        }
    }

    println!("{}", final_data.len());
    final_data
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
