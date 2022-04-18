const BASE_CUR_URL: &str = "https://api.curseforge.com";
const BASE_MRN_URL: &str = "https://api.modrinth.com/api/v1/mod";
const BASE_MRN_URL2: &str = "https://api.modrinth.com/v2";


use crate::minecraft_mod::*;
pub struct ModRinth {}

impl ModRinth {
    pub fn search() -> String {
        format!("{}", BASE_MRN_URL)
    }

    pub fn search_for(limit: u32, offset: u32) -> String {
        format!("{}/search?limit={}&offset={}", BASE_MRN_URL2, limit, offset)
    }

    pub fn get_mod_info_by_id(id: &str) -> String {
        // https://api.modrinth.com/v2/project/AANobbMI
        format!(
            "{}/project/{}",
            BASE_MRN_URL2,
            id
        )
    }

    pub fn mod_versions(minecraft_mod: &RinthMod) -> String {
        // https://api.modrinth.com/v2/project/AANobbMI/version
        format!(
            "{}/project/{}/version",
            BASE_MRN_URL2,
            minecraft_mod.get_id()
        )
    }

    pub fn mod_versions_by_id(id: &str) -> String {
        // https://api.modrinth.com/v2/project/AANobbMI/version
        format!(
            "{}/project/{}/version",
            BASE_MRN_URL2,
            id
        )
    }

    pub fn querry(q: &str) -> String {
        format!("{}/search?query={}", BASE_MRN_URL2, q)
    }

    pub fn hash(hash: &str) -> String {
        format!("{}/version_file/{}", BASE_MRN_URL2, hash)
    }
}
pub struct Curse {
    // TODO

}

impl Curse {
    pub fn search_for() -> String {
        format!("{}/v1/mods/search?gameid=432&pageSize=20&classId=6", BASE_CUR_URL)
    }
}

/*
pub fn curse_games() -> String{
    format!("{}/v1/categories", BASE_CUR_URL)
}
*/
