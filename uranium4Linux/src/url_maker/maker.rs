#![allow(dead_code)]
const BASE_CUR_URL: &str = "https://api.curseforge.com";
const BASE_MRN_URL: &str =  "https://api.modrinth.com/api/v1/mod";
const BASE_MRN_URL2: &str = "https://api.modrinth.com/v2";

use minecraft_mod::minecraft_mod::*;
use minecraft_mod::responses::*;
pub struct ModRinth{

}


impl ModRinth {
    
    pub fn search() -> String{
        format!("{}", BASE_MRN_URL)
    }
    
    pub fn search_for(limit: u32, offset: u32) -> String{
        format!("{}/search?limit={}&offset={}", BASE_MRN_URL2, limit, offset)
    }

    pub fn mod_versions(minecraft_mod: &RinthMod) -> String{
        // https://api.modrinth.com/api/v1/mod/AANobbMI/version
        // https://api.modrinth.com/v2/project/AANobbMI/version
        format!("{}/project/{}/version", BASE_MRN_URL2, minecraft_mod.get_id())
    }

    pub fn querry(q: &String) -> String{
        format!("{}/search?query={}", BASE_MRN_URL2, q)
    }
    
    pub fn hash(hash: &String) -> String {
        format!("{}/version_file/{}", BASE_MRN_URL2, hash)
    }

}
pub struct Curse{

}

impl Curse{
    pub fn search_for() -> String{ 
        format!("{}/v1/mods/search?gameid=432&pageSize=20&classId=6", BASE_CUR_URL)
    }
}



/*
pub fn curse_games() -> String{
    format!("{}/v1/categories", BASE_CUR_URL)
}
*/
