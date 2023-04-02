#![allow(non_snake_case)]
#![allow(dead_code)]

use super::rinth_mods::{Hashes, RinthVersion};
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
    pub fn new() -> RinthModpack {
        RinthModpack {
            formatVersion: 1,
            game: "minecraft".to_owned(),
            versionId: "0.0.0".to_owned(),
            name: "example".to_owned(),
            files: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_files(&self) -> &Vec<RinthMdFiles> {
        &self.files
    }

    pub fn add_mod(&mut self, new_mod: RinthMdFiles) {
        self.files.push(new_mod);
    }

    pub fn write_mod_pack_with_name(&self) {
        let j = serde_json::to_string_pretty(self).unwrap();
        std::fs::write("modrinth.index.json", j).unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RinthMdFiles {
    path: String,
    hashes: Hashes,
    downloads: Vec<String>,
    fileSize: usize,
}

impl std::convert::From<RinthVersion> for RinthMdFiles {
    fn from(version: RinthVersion) -> RinthMdFiles {
        RinthMdFiles {
            path: "mods/".to_owned() + &version.get_file_name(),
            hashes: version.get_hashes().clone(),
            downloads: vec![version.get_file_url()],
            fileSize: version.get_size(),
        }
    }
}

impl RinthMdFiles {
    pub fn get_download_link(&self) -> String {
        self.downloads[0].clone()
    }

    pub fn get_download_link_raw(&self) -> &str {
        &self.downloads[0]
    }

    pub fn get_id(& self) -> Option<&str> {
        for download_link in &self.downloads {
            if download_link.contains("modrinth") {
                return download_link.split("data/")
                    .nth(1)
                    .map(|f| 
                        &f[0..8]
                    );
            }
        } 
        None
    }

    pub fn get_name(&self) -> String {
        self.path.strip_prefix("mods/").unwrap().to_owned()
    }

    pub fn get_raw_name(&self) -> &str {
        self.path.strip_prefix("mods/").unwrap()
    }
}

fn deserializ_pack(path: &str) -> RinthModpack {
    serde_json::from_str(&read_to_string(path).unwrap()).unwrap()
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
