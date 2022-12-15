use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BASE: &'static str = "http://resources.download.minecraft.net/";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectData {
    pub hash: Option<String>,
    pub size: Option<usize>,
}

impl ObjectData {
    pub fn get_link(&self) -> String { 
        match &self.hash {
            Some(e) => format!("{}{}/{}", BASE, &e[..2], e),
            None => String::new(),
        }
       
        //format!("{}{}/{}", BASE, &self.hash[..2], self.hash)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadData<'a> {
    sha1: &'a str,
    size: usize,
    url: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Objects {
    #[serde(flatten)]
    pub files: HashMap<String, ObjectData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    pub objects: Objects
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance<'a> {
    id: &'a str,
    downloads: HashMap<&'a str, DownloadData<'a>>,
}
