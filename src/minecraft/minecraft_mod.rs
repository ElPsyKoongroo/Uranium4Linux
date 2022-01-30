#![allow(dead_code)]
use core::fmt;
use std::fmt::format;

use serde::{Serialize, Deserialize};

use crate::requester::request_maker;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RinthMod{
    mod_id: String,
    title: String,
    latest_version: String,
    downloads: u32,
    versions: Vec<String>,
    categories: Vec<String>
}

impl RinthMod {
    pub fn get_id(&self) -> String{
        let id = self.mod_id.clone().split_off(6);
        id
    }

    pub fn to_string(&self) -> String{
        format!("Mod name: {}", self.title)

    }

    pub fn get_versions(&self) -> &Vec<String> {
        &self.versions
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RinthVersion{
    id: String,
    mod_id: String,
    name: String,
    version_type: String,
    downloads: u64,
    files: Vec<RinthFile>,
    dependencies: Vec<String>
}

impl RinthVersion {
    
    pub fn get_file_url(&self) -> String{
        self.files[0].url.clone()
    }

    pub fn get_file_name(&self) -> String {
        self.files[0].filename.clone()
    }


}


#[derive(Debug, Serialize, Deserialize)]
pub struct RinthVersions {
    pub versions: Vec<RinthVersion>
}


impl RinthVersions {

    pub fn get_version(&self, i: usize) -> &RinthVersion{
        &self.versions[i]
    }


}

impl fmt::Display for RinthVersions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut max_width = {
            &self.versions.iter()
            .map(|x| x.name.clone().len())
            .max()
            .unwrap_or(0)
        };
        let vt_len = "version type".chars().count();

        if max_width<&vt_len{
            max_width = &vt_len;
        }

        write!(f, "\n\nindex\t{:<max_width$}\tversion type\tdownloads\n", "version name")?;
        for (index, version) in self.versions.iter().enumerate(){
            write!(f, "{index:^5}\t{:<max_width$}\t{:^12}\t{:^9}\n", version.name, version.version_type, version.downloads)?;
        }

        write!(f, "")

    }
}


#[derive(Debug, Serialize, Deserialize)]

struct RinthFile{
    pub url: String,
    pub filename: String
}


