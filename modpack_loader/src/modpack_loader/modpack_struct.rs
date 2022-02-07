#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ModPack{
    pub count: usize,
    pub name: String,
    pub version: String,
    pub author: String,
    pub mods: Vec<Mods>,
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
}