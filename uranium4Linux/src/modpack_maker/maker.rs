use std::{fs::{self, read_dir}, io::Read, path::Path};
use hex::ToHex;
use mine_data_strutcs::{minecraft_mod::{RinthVersions, RinthVersion}, url_maker::maker};
use requester::requester::request_maker::Requester;
use sha1::{Sha1, Digest};
use crate::{easy_input, checker::check};

pub async fn make_modpack(path: &str) {
    let mut requester = Requester::new();
    let hash_filename = get_mods(Path::new(path)).unwrap();
    let mut responses: RinthVersions = RinthVersions::new();
    search_mods_for_modpack(&mut requester, hash_filename, &mut responses).await;
    
    let mp_name = easy_input::input("Modpack name: ", String::from("Modpack.mm"));
    let mp_version = easy_input::input("Modpack version: ", String::from("1.0"));
    let mp_author = easy_input::input("Modpack author: ", String::from("Anonimous"));
    let mp =
        mine_data_strutcs::modpack_struct::ModPack::modpack_from_RinthVers(mp_name, mp_version, mp_author, responses);
    mp.write_mod_pack();

    let _ = easy_input::input("Press enter to continue...", 0);
}

fn get_sha1_from_file(file_path: &String) -> String {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path).unwrap();
    let metadata = fs::metadata(file_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    hasher.update(buffer);
    let temp = hasher.finalize().to_vec();
    let hash = temp.encode_hex::<String>();
    hash
}

fn get_mods(mods_path: &Path) -> Option<Vec<(String, String)>> {
    let mut names: Vec<(String, String)> = Vec::new();
    let mods;

    if !mods_path.is_dir() {return None;}
    
    match read_dir(mods_path) {
        Ok(e) => mods = e,
        Err(error) => {
            eprintln!("Error reading the directore: {}", error);
            return None
        }
    }

    for mmod in mods {
        get_sha(mods_path, mmod.unwrap(), &mut names);
    }

    Some(names)
}

fn get_sha(path: &Path, mod_dir: fs::DirEntry, names_vec: &mut Vec<(String, String)>) {
    let file_name = mod_dir.file_name().into_string().unwrap();
    let file_path = { path.join(&file_name).to_str().unwrap().to_string() };
    let hash = get_sha1_from_file(&file_path);
    names_vec.push((hash, file_name));
}

async fn search_mods_for_modpack(requester: &mut Requester, hash_filename: Vec<(String, String)>, responses: &mut RinthVersions) {
    for item in hash_filename {
        let response = {
            let request = requester.get(maker::ModRinth::hash(&item.0)).await.unwrap();
            check(
                request.json::<RinthVersion>().await,
                false,
                true,
                format!("Mod {} was not found !", &item.1).as_str(),
            )
        };
        match response {
            Some(e) => responses.push(e),
            None => {}
        }
    }
}

