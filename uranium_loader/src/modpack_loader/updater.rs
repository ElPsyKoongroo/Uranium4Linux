use mine_data_strutcs::minecraft_mod::RinthVersions;
use mine_data_strutcs::modpack_mod::Mods;
use mine_data_strutcs::modpack_struct::{load_pack, ModPack};
use mine_data_strutcs::url_maker;

use std::collections::VecDeque;

use regex::Regex;
use requester::requester::request_maker::Requester;
use std::cmp;

pub async fn update_modpack(modpack_path: String) {
    println!("Updater is not implemented yet");
    let old_modpack: ModPack = load_pack(&modpack_path).unwrap();
    let identifiers = get_project_identifiers(&old_modpack);
    
    let mut mods_to_update: VecDeque<Mods> = get_updates(modpack_path, &identifiers).await;
    
    
    let mut updated_modpack = ModPack::new();
    make_updates(&old_modpack, &mut mods_to_update, &mut updated_modpack);

    updated_modpack.set_name(old_modpack.get_name());
    updated_modpack.set_version(old_modpack.get_version());
    updated_modpack.write_mod_pack();
}

fn make_updates(
    pack: &ModPack,
    mods_to_update: &mut VecDeque<Mods>,
    updated_modpack: &mut ModPack,
) {
    for mine_mod in pack.mods() {
        if mine_mod.get_id() == mods_to_update.front().unwrap().get_id() {
            println!("Updating mod: {}", mine_mod.get_name());
            updated_modpack.push_mod(mods_to_update.pop_front().unwrap());
        } else {
            updated_modpack.push_mod(mine_mod.clone());
        }

        if mods_to_update.len() == 0 {
            break;
        }
    }
}

async fn get_updates(modpack_path: String, identifiers: &Vec<String>) -> VecDeque<Mods> {
    let mpack_mods: ModPack = load_pack(&modpack_path).unwrap();
    let mut mods_info: RinthVersions = RinthVersions::new();
    let mut updated_mods: VecDeque<Mods> = VecDeque::new();

    get_new_versions(identifiers, &mut mods_info).await;
    let _max_len = cmp::min(mpack_mods.len(), mods_info.len());

    for i in 0..mods_info.len() {
        if mpack_mods.mods()[i].get_file() != mods_info.mod_at(i).get_file_url()
            && mods_info.mod_at(i).get_loader() == "fabric"
        {
            updated_mods.push_back(Mods::from_RinthVersion(mods_info.mod_at(i).clone()));
        }
    }
    updated_mods
}

async fn get_new_versions(identifiers: &Vec<String>, mods_info: &mut RinthVersions) {
    let req = Requester::new();
    for id in identifiers.iter() {
        let url = url_maker::maker::ModRinth::mod_versions_by_id(&id);
        let mod_info: RinthVersions = req.get(url).await.unwrap().json().await.unwrap();
        mods_info.push(mod_info.mod_at(0).clone());
    }
}

fn get_project_identifiers(modpack: &ModPack) -> Vec<String> {
    let re = Regex::new("data/(.{8})").unwrap();
    let mut identifiers = Vec::new();

    for minecraft_mod in modpack.mods() {
        for cap in re.captures_iter(minecraft_mod.get_file().as_str()) {
            identifiers.push(cap[1].to_string());
        }
    }
    identifiers
}

