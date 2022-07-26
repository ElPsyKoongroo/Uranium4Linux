use crate::{checker::check, easy_input, zipper::pack_zipper::compress_pack};
use std::fs::read_dir;
use mine_data_strutcs::{
    rinth::rinth_mods::{RinthVersion, RinthVersions},
    curse::curse_mods::{CurseFingerPrint, CurseResponse},
    url_maker::maker,
};
use requester::requester::request_maker::{Requester, CurseRequester, CurseMethod};
use std::path::Path;
use crate::hashes::{rinth_hash, curse_hash};

struct ModHashes{
    rinth_hash: String,
    curse_hash: String
}

pub async fn make_modpack(path: &str) {
    let mut requester = Requester::new();
    let hash_filename = get_mods(Path::new(path)).unwrap();

    let mut responses: RinthVersions = RinthVersions::new();
    let mut not_found_mods: Vec<String> = Vec::new();
    search_mods_for_modpack(
        &mut requester,
        hash_filename,
        &mut responses,
        &mut not_found_mods,
    ).await;

    let mp_name = easy_input::input("Modpack name: ", String::from("Modpack.mm"));
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
    let _ = easy_input::input("Press enter to continue...", 0);
}


fn get_mods(minecraft_path: &Path) -> Option<Vec<(ModHashes, String)>> {
    let mut hashes_names = Vec::new();
    let mods;

    if !minecraft_path.is_dir() {
        return None;
    }
    let mods_path = minecraft_path.join("mods/");

    match read_dir(&mods_path) {
        Ok(e) => mods = e.into_iter()
            .map(|f| f.unwrap().path().to_str().unwrap().to_owned())
            .collect::<Vec<String>>(),
        Err(error) => {
            eprintln!("Error reading the directore: {}", error);
            return None;
        }
    }

    // Push all the (has, file_name) to the vector
    for path in mods {
        let rinth = rinth_hash(&path);
        let curse = curse_hash(&path);
        let hashes = ModHashes{
            rinth_hash: rinth,
            curse_hash: curse
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
    requester: &mut Requester,
    hash_filename: Vec<(ModHashes, String)>,
    responses: &mut RinthVersions,
    not_found_mods: &mut Vec<String>,
){
    for item in hash_filename {
        let response = search_mod(requester, &item).await;
        match response {
            Some(e) => responses.push(e),
            None => not_found_mods.push(item.1),
        }
    }    
}


async fn search_mod(requester: &Requester, item: &(ModHashes, String)) -> Option<RinthVersion>{
    let response = {
        let rinth_request = requester.get(maker::ModRinth::hash(&item.0.rinth_hash)).await.unwrap();

        let curse_body = format!("{{
                \"fingerprints\": [
                    {}
                ]
            }}", item.0.curse_hash
        );
        
        
        let curse_requester = CurseRequester::new();
        let url = maker::Curse::hash();
        let curse_request = curse_requester.get(url.clone(), CurseMethod::POST, &curse_body).await.await.unwrap().unwrap();

        let curse_text = curse_request.text().await.unwrap();
        let curse_parse: Result<CurseResponse<CurseFingerPrint>, _> = serde_json::from_str(&curse_text); 
        /*
        let curse_parse = check(
            curse_request.json::<CurseFingerPrint>().await,
            false,
            false,
            ""
        );
        */
        let rinth_parse = check(
            rinth_request.json::<RinthVersion>().await,
            false,
            false,
            "",
        );    
        if rinth_parse.is_some(){
            rinth_parse
        } 
        else if curse_parse.is_ok(){
            let rinth_parse = RinthVersion::from_CurseFile(curse_parse.unwrap().data.get_file().clone());
            // Sometimes even if CurseApi has the mod in the database the URL field can be 
            // empty so we need to check firt
            if rinth_parse.get_file_url() != ""{
                Some(rinth_parse)
            } else {
                None
            }
        } else {println!("[Maker {}]\nMod {} was not found !\n", 106, &item.1); None}
    };
    response 
}

fn fix_name(name: &mut String) {
    if name.ends_with(".json") {
        name.strip_suffix(".json").unwrap();
    }
    name.push_str("_temp.json");
}
