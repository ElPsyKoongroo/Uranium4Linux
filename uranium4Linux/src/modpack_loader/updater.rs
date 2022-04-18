use mine_data_strutcs::minecraft_mod::{RinthVersion, RinthVersions};
use mine_data_strutcs::modpack_mod::Mods;
use mine_data_strutcs::modpack_struct::{load_pack, ModPack};
use mine_data_strutcs::url_maker;
use regex::Regex;
use requester::async_pool::AsyncPool;
use std::collections::VecDeque;
use tokio::task;

pub async fn update_modpack(modpack_path: &str) {
    let old_modpack: ModPack = load_pack(modpack_path).unwrap();
    let identifiers = get_project_identifiers(&old_modpack);

    let mut mods_to_update: VecDeque<Mods> = get_updates(&identifiers).await;

    let mut updated_modpack = ModPack::new();
    make_updates(&mut mods_to_update, &mut updated_modpack);

    updated_modpack.set_name(old_modpack.get_name());
    updated_modpack.set_version(old_modpack.get_version());
    updated_modpack.write_mod_pack();
}

/// Update the old versions of the mods with the new ones
fn make_updates(mods_to_update: &mut VecDeque<Mods>, updated_modpack: &mut ModPack) {
    mods_to_update
        .iter()
        .for_each(|m| updated_modpack.push_mod(m.clone()));
}

/// Sorts the modpack mods by their identifiers
fn sort_mods(mods: &mut RinthVersions, identifiers: &Vec<String>) -> RinthVersions {
    let mut sorted_mods: RinthVersions = RinthVersions::new();

    for identifier in identifiers {
        for mod_ in mods.mods() {
            if mod_.get_project_id() == *identifier {
                sorted_mods.push(mod_.clone());
            }
        }
    }
    sorted_mods
}

/// Compare every mod of the old modpack with the last version found.
async fn get_updates(identifiers: &Vec<String>) -> VecDeque<Mods> {
    let mut mods_lastests_versions: RinthVersions = RinthVersions::new();
    let mut updated_mods: VecDeque<Mods> = VecDeque::new();

    get_new_versions(identifiers, &mut mods_lastests_versions).await;
    mods_lastests_versions = sort_mods(&mut mods_lastests_versions, identifiers);

    for i in 0..mods_lastests_versions.len(){
        updated_mods.push_back(Mods::from_RinthVersion(
            mods_lastests_versions.mod_at(i).clone(),
        ));
    }
    updated_mods
}

#[allow(dead_code)]
/// True if old_mod is not the lastest version of the mod
fn is_newest(old_mod: &Mods, new_mod: &Mods) -> bool {
    return old_mod.get_file() != new_mod.get_file();
}

/// Get the latest versions of all the idetifiers
async fn get_new_versions(identifiers: &Vec<String>, mods_info: &mut RinthVersions) {
    let mut pool = AsyncPool::new();
    for id in identifiers.iter() {
        let url = url_maker::maker::ModRinth::mod_versions_by_id(&id);
        let a_func = async {
            let cliente = reqwest::Client::new();
            cliente.get(url).send().await.unwrap()
        };
        let task = task::spawn(a_func);
        pool.push_request(task);
    }

    pool.start().await;
    let done_responses = pool.get_done_request();
    for i in done_responses {
        let value = i.text().await.unwrap();
        let a: Result<Vec<RinthVersion>, serde_json::Error> = serde_json::from_str(value.as_str());
        match a{
            Ok(t) => {
                mods_info.push(t[0].clone());
            }
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("{:?}", value.get(12300..12500));
            }
        }
    }
}

fn get_project_identifiers(modpack: &ModPack) -> Vec<String> {
    let re = Regex::new("data/(.{8})").unwrap(); // MAGIC !!
    let mut identifiers = Vec::new();

    for minecraft_mod in modpack.mods() {
        for cap in re.captures_iter(minecraft_mod.get_file().as_str()) {
            identifiers.push(cap[1].to_owned())
        }
    }
    identifiers
}
