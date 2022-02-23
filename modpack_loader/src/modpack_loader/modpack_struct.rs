#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use minecraft_mod::minecraft_mod::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct ModPack{
    name: String,
    version: String,
    author: String,
    mods: Vec<Mods>,
    count: usize,
}
impl ModPack{
    pub fn new() -> ModPack{
        ModPack{
            count: 0,
            name: String::from(" "),
            version: String::from(" "),
            author: String::from(" "),
            mods: Vec::new()

        }
    }
    pub fn mods(&self) -> &Vec<Mods>{
        &self.mods
    }
    #[allow(non_snake_case)]
    pub fn modpack_from_RinthVers(mods: Vec<RinthVersion>)->ModPack{
        let mut mod_vec = Vec::new();
        for mmod in mods{
            mod_vec.push(Mods::from_RinthVersion(mmod));
        }
        ModPack{
            count: mod_vec.len(),
            name: String::from("Modpack_1"),
            version: String::from("1.0"),
            author: String::from("Author"),
            mods: mod_vec,
        }
    }

    pub fn write_mod_pack(&self){
        let j = serde_json::to_string(self).unwrap();
        std::fs::write(self.name.clone(), j).unwrap();
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
    file_name: String
}

impl Mods{
    pub fn new(_name:String, _file:String, _file_name:String) -> Mods{
        Mods { name: (_name), file: (_file), file_name: (_file_name) }
    }
    
    pub fn get_file(&self) -> String{
        self.file.clone()
    }

    pub fn get_file_name(&self) -> String{
        self.file_name.clone()
    }
    
    #[allow(non_snake_case)]
    pub fn from_RinthVersion(m_mod: RinthVersion) -> Mods{
        Mods::new(
            m_mod.get_name(), 
            m_mod.get_file_url(), 
            m_mod.get_file_name())
    }
}