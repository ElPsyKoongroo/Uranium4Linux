#![allow(dead_code)]
use mine_data_strutcs::minecraft_mod::*;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct ModPack {
    name: String,
    version: String,
    author: String,
    mods: Vec<Mods>,
    count: usize,
}
impl ModPack {
    pub fn new() -> ModPack {
        ModPack {
            count: 0,
            name: String::from(" "),
            version: String::from(" "),
            author: String::from(" "),
            mods: Vec::new(),
        }
    }

    pub fn mods(&self) -> &Vec<Mods> {
        &self.mods
    }

    #[allow(non_snake_case)]
    pub fn modpack_from_RinthVers(
        modpack_name: String,
        modpack_version: String,
        modpack_author: String,
        mods: Vec<RinthVersion>,
    ) -> ModPack {
        let mut mod_vec = Vec::new();
        for mmod in mods {
            mod_vec.push(Mods::from_RinthVersion(mmod));
        }
        ModPack {
            count: mod_vec.len(),
            name: modpack_name,
            version: modpack_version,
            author: modpack_author,
            mods: mod_vec,
        }
    }

    pub fn write_mod_pack(&self) {
        let j = serde_json::to_string(self).unwrap();
        std::fs::write(self.name.clone(), j).unwrap();
    }
}

fn deserializ_pack(path: String) -> Result<ModPack, Error> {
    let j = fs::read_to_string(path).unwrap();
    let pack: ModPack = serde_json::from_str(&j).unwrap();
    Ok(pack)
}

pub fn load_pack(pack_path: &String) -> Option<ModPack> {
    match fs::read_to_string(pack_path) {
        Ok(_) => {}
        Err(error) => {
            println!("Error reading the pack \n\n{error}");
            return None;
        }
    };
    match deserializ_pack(pack_path.clone()) {
        Ok(e) => return Some(e),
        Err(error) => {
            println!("Error deserializing the pack \n\n{error}");
            return None;
        }
    }
}

impl Iterator for ModPack {
    type Item = Mods;
    fn next(&mut self) -> Option<Mods> {
        // Increment our count. This is why we started at zero.
        self.count += 1;

        // Check to see if we've finished counting or not.
        if self.count < self.mods.len() {
            Some(self.mods[self.count].clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Mods {
    name: String,
    file: String,
    file_name: String,
}

impl Mods {
    pub fn new(_name: String, _file: String, _file_name: String) -> Mods {
        Mods {
            name: (_name),
            file: (_file),
            file_name: (_file_name),
        }
    }

    pub fn get_file(&self) -> String {
        self.file.clone()
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.clone()
    }

    #[allow(non_snake_case)]
    pub fn from_RinthVersion(m_mod: RinthVersion) -> Mods {
        Mods::new(
            m_mod.get_name(),
            m_mod.get_file_url(),
            m_mod.get_file_name(),
        )
    }
}
