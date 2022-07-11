use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs;

use crate::{rinth_api::RinthVersions, modpack_mod::*};

#[derive(Serialize, Deserialize, Clone)]
pub struct ModPack {
    name: String,
    version: String,
    author: String,
    mods: Vec<Mods>,
}

impl ModPack {
    pub fn new() -> ModPack {
        ModPack {
            name: String::from(" "),
            version: String::from(" "),
            author: String::from(" "),
            mods: Vec::new(),
        }
    }

    #[allow(non_snake_case)]
    pub fn modpack_from_RinthVers(
        modpack_name: &str,
        modpack_version: String,
        modpack_author: String,
        mods: RinthVersions,
    ) -> ModPack {
        let mod_vec = mods
            .versions
            .iter()
            .map(|x| Mods::from_RinthVersion(x.clone()))
            .collect::<Vec<Mods>>();

        ModPack {
            name: modpack_name.to_owned(),
            version: modpack_version,
            author: modpack_author,
            mods: mod_vec,
        }
    }

    pub fn mods(&self) -> &Vec<Mods> {
        &self.mods
    }

    pub fn write_mod_pack(&self) {
        let j = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(self.name.clone(), j).unwrap();
    }

    pub fn write_mod_pack_with_name(&self, name: &str) {
        let j = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(name, j).unwrap();
    }

    pub fn push_mod(&mut self, mine_mod: Mods) {
        self.mods.push(mine_mod);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    pub fn len(&self) -> usize {
        self.mods.len()
    }

    pub fn mod_at(&self, i: usize) -> &Mods {
        &self.mods[i]
    }
}

fn deserializ_pack(path: &str) -> Result<ModPack, Error> {
    let j = fs::read_to_string(path).unwrap();
    let pack: ModPack = serde_json::from_str(&j).unwrap();
    Ok(pack)
}

pub fn load_pack(pack_path: &str) -> Option<ModPack> {
    match fs::read_to_string(pack_path) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error reading the pack \n\n{error}");
            return None;
        }
    };

    match deserializ_pack(pack_path) {
        Ok(e) => return Some(e),
        Err(error) => {
            eprintln!("Error deserializing the pack \n\n{error}");
            return None;
        }
    }
}
