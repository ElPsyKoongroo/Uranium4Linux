#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::rinth::rinth_mods::RinthProject;

const BASE_CUR_URL: &str = "https://api.curseforge.com";
const BASE_MRN_URL: &str = "https://api.modrinth.com/api/v1/mod";
const BASE_MRN_URL2: &str = "https://api.modrinth.com/v2";

pub struct ModRinth;

impl ModRinth {
    pub fn search() -> String {
        BASE_MRN_URL2.to_string()
    }

    pub fn search_for(limit: u32, offset: u32) -> String {
        format!("{}/search?limit={}&offset={}", BASE_MRN_URL2, limit, offset)
    }

    pub fn get_project_by_id(id: &str) -> String {
        // https://api.modrinth.com/v2/project/6AQIaxuO
        format!("{}/project/{}", BASE_MRN_URL2, id)
    }

    pub fn mod_versions(minecraft_mod: &RinthProject) -> String {
        // https://api.modrinth.com/v2/project/AANobbMI/version
        format!(
            "{}/project/{}/version",
            BASE_MRN_URL2,
            minecraft_mod.get_id()
        )
    }

    /// https://api.modrinth.com/v2/project/AANobbMI/version
    pub fn mod_versions_by_id(id: &str) -> String {
        format!("{}/project/{}/version", BASE_MRN_URL2, id)
    }

    pub fn resourcepacks(limit: u32, offset: u32) -> String {
        format!(
            "{}/search?limit={}&offset={}&facets=[[\"project_type:resourcepack\"]]",
            BASE_MRN_URL2, limit, offset,
        )
    }

    pub fn modpacks(limit: u32, offset: u32) -> String {
        format!(
            "{}/search?limit={}&offset={}&facets=[[\"project_type:modpack\"]]",
            BASE_MRN_URL2, limit, offset,
        )
    }

    pub fn mod_version_by_id(id: &str) -> String {
        // https://api.modrinth.com/v2/version/{id}
        format!("{}/version/{}", BASE_MRN_URL2, id)
    }

    pub fn querry(q: &str) -> String {
        format!("{}/search?query={}", BASE_MRN_URL2, q)
    }

    pub fn hash(hash: &str) -> String {
        format!("{}/version_file/{}", BASE_MRN_URL2, hash)
    }

    pub fn update_by_hash_post() -> String {
        format!("{}/version_files/update", BASE_MRN_URL2)
    }
}

pub struct Curse;

impl Curse {
    pub fn file(modId: &str, fileId: &str) -> String {
        format!("{}/v1/mods/{}/files/{}", BASE_CUR_URL, modId, fileId)
    }

    pub fn hash() -> String {
        format!("{}/v1/fingerprints", BASE_CUR_URL)
    }
}
