#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::read_to_string;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CursePackFiles {
    projectID: usize,
    fileID: usize,
}

impl CursePackFiles {
    pub fn get_projectID(&self) -> usize {
        self.projectID
    }

    pub fn get_fileID(&self) -> usize {
        self.fileID
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CursePack {
    name: String,
    author: String,
    files: Vec<CursePackFiles>
}

impl CursePack{
    pub fn get_files(&self) -> &Vec<CursePackFiles> {
        &self.files
    }
}

fn deserializ_pack(path: &str) -> Result<CursePack, Error> {
    let aux = read_to_string(path).unwrap();
    serde_json::from_str(&aux)
}

pub fn load_curse_pack(pack_path: &str) -> Option<CursePack> {
    match read_to_string(pack_path) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error reading the pack \n\n{error}");
            return None;
        }
    };

    match deserializ_pack(pack_path) {
        Ok(e) => Some(e),
        Err(error) => {
            eprintln!("Error deserializing the pack \n\n{error}");
            None
        }
    }
}
