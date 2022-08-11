use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::{rinth::rinth_mods::*, curse::curse_mods::{CurseVersion, CurseFile}};

#[derive(Clone, Deserialize, Serialize)]
pub enum Repo{
    RINTH,
    CURSE
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Mods {
    repo: Repo,
    file: String,
    file_name: String,
}

impl Mods {
    pub fn new(repo: Repo, file: String, file_name: String) -> Mods {
        Mods {
            repo,
            file,
            file_name,
        }
    }

    pub fn get_file(&self) -> String {
        self.file.clone()
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.clone()
    }

    /*
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    */

    pub fn get_id(&self) -> String {
        let re = Regex::new("data/(.{8})").unwrap();
        let file_url = &self.file;
        let a = re.captures(file_url.as_str()).unwrap();
        a[1].to_string()
    }

    #[allow(non_snake_case)]
    pub fn from_RinthVersion(m_mod: &RinthVersion) -> Mods {
        Mods::new(
            Repo::RINTH,
            m_mod.get_file_url(),
            m_mod.get_file_name(),
        )
    }
    
    #[allow(non_snake_case)]
    pub fn from_CurseVersion(m_mod: &CurseFile) -> Mods {
        Mods::new(
            Repo::CURSE,
            m_mod.get_id().to_string(),
            m_mod.get_fileName(),
        )
    }
}

