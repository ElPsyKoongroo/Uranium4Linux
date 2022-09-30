#![allow(non_snake_case)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RinthModpack {
    formatVersion: usize,
    game: String,
    versionId: String,
    name: String,
    files: Vec<RinthMdFiles>,
}

impl RinthModpack {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_files(&self) -> &Vec<RinthMdFiles> {
        &self.files
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RinthMdFiles {
    path: String,
    //hashes: Vec<String>,
    downloads: Vec<String>,
    fileSize: usize,
}

impl RinthMdFiles {
    pub fn get_download_link(&self) -> String {
        self.downloads[0].clone()
    }

    pub fn get_name(&self) -> String {
        self.path.strip_prefix("mods/").unwrap().to_owned()
    }
}


fn deserializ_pack(path: &str) -> RinthModpack {
    let j = read_to_string(path).unwrap();
    let pack: RinthModpack = serde_json::from_str(&j).unwrap();
    pack
}

pub fn load_rinth_pack(pack_path: &str) -> RinthModpack {
    match read_to_string(pack_path) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error reading the pack \n\n{error}");
            panic!();
        }
    };

    deserializ_pack(pack_path) 
}
