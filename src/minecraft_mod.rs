#![allow(dead_code)]
use std::fmt::format;

use serde::{Serialize, Deserialize};

trait Downloadeable{
    fn download(){}
}



#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CurseMod{
    id: u64,
    name: String,
    downloadCount: f64
}



//
//
// RINTH MODS
//
//

#[derive(Serialize, Deserialize, Debug)]
pub struct RinthMod{
    mod_id: String,
    title: String,
    latest_version: String,
    downloads: u32,
    versions: Vec<String>,
    categories: Vec<String>
}

struct RinthVersions{
    id: String,
    mod_id: String,
    name: String,
    version_type: String,
    downloads: u64,
    files: Vec<RinthFile>,
    dependencies: Vec<String>
}

struct RinthFile{
    pub url: String,
    pub filename: String
}

impl RinthMod {
    pub fn get_id(&self) -> String{
        let id = self.mod_id.clone().split_off(6);
        id
    }

    pub fn to_string(&self) -> String{
        format!("Mod name: {}", self.title)

    }
}
