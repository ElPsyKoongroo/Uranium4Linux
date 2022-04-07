#![allow(dead_code)]
use regex::Regex;
use super::modpack_struct::*;
use mine_data_strutcs::{url_maker, minecraft_mod::RinthMod};
use mine_data_strutcs::minecraft_mod::RinthVersion;
use requester::requester::request_maker::Requester;
use std::cmp;

pub async fn update_modpack(modpack_path: String) {
    println!("Updater is not implemented yet");
    let pack: ModPack = load_pack(&modpack_path).unwrap();
    let re = Regex::new("data/(.{8})").unwrap();
    let mut identifiers = Vec::new();


    for mmod in pack.mods() {
        for cap in re.captures_iter(mmod.get_file().as_str()){
            identifiers.push(cap[1].to_string());
        }
    }
    get_updates(modpack_path).await;
    
}

pub fn get_project_identifiers(modpack_path: String) -> Vec<String>{
    println!("Updater is not implemented yet");
    let pack: ModPack = load_pack(&modpack_path).unwrap();
    let re = Regex::new("data/(.{8})").unwrap();
    let mut identifiers = Vec::new();


    for mmod in pack.mods() {
        for cap in re.captures_iter(mmod.get_file().as_str()){
            identifiers.push(cap[1].to_string());
        }
    }
    identifiers
}

async fn check_versions(mod_versions: &Vec<RinthVersion>, mine_mod: &Mods, versions_id: &mut Vec<String>){
    for version in mod_versions {
        if version.get_file_url() == mine_mod.get_file() {
            println!("Version found!");
            versions_id.push(version.get_id());
            break;
        }
    }
}


async fn get_new_versions(identifiers: Vec<String>, mods_info: &mut Vec<RinthMod>, mpack_mods: ModPack, versions_id: &mut Vec<String>){
    let req = Requester::new();
    for (i, id) in identifiers.iter().enumerate(){
        let url = url_maker::maker::ModRinth::get_mod_info_by_id(&id);
        let mod_info: RinthMod = req.get(url).await.unwrap().json().await.unwrap();
        mods_info.push(mod_info);

        let mod_versions: Vec<RinthVersion> = req.get(
            url_maker::maker::ModRinth::mod_versions_by_id(&id)
        ).await.unwrap().json().await.unwrap();

        check_versions(&mod_versions, &mpack_mods.mods()[i], versions_id).await;
    }    
}

async fn get_updates(modpack_path: String) {
    let mpack_mods: ModPack = load_pack(&modpack_path).unwrap();
    let identifiers = get_project_identifiers(modpack_path);
    let mut mods_info: Vec<RinthMod> = Vec::new();
    let mut versions_id: Vec<String> = Vec::new();
    
    get_new_versions(identifiers, &mut mods_info, mpack_mods, &mut versions_id).await;
    let max_len = cmp::min(versions_id.len(), mods_info.len());

    for i in 0..max_len {
        if mods_info[i].get_versions()[0] != versions_id[i] {
            println!("New version avaliable for {}", mods_info[i].get_title());
        } else {
            println!("{} is up to date !!", mods_info[i].get_title());
        }
    }
}


/*


let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let text = "2012-03-14, 2013-01-01 and 2014-07-05";
for cap in re.captures_iter(text) {
    println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
}
// Output:
// Month: 03 Day: 14 Year: 2012
// Month: 01 Day: 01 Year: 2013
// Month: 07 Day: 05 Year: 2014


*/