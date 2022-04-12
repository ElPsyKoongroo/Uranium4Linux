use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::minecraft_mod::RinthVersion;

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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_id(&self) -> String {
        let re = Regex::new("data/(.{8})").unwrap();
        let file_url = self.get_file();
        let a = re.captures(file_url.as_str()).unwrap();
        a[1].to_string()
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