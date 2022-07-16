#![allow(non_snake_case)]
#![allow(dead_code)] 
use serde::{Deserialize, Serialize}; 


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RinthModpack{
    formatVersion: usize,
    game: String,
    versionId: String,
    name: String,
    summary: String,
    files: Vec<RinthMdFiles>
}

impl RinthModpack{

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_files(&self) -> &Vec<RinthMdFiles> {
        &self.files
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RinthMdFiles{
    path: String,
    hashes: Vec<String>,
    downloads: Vec<String>,
    fileSize: usize,
}

impl RinthMdFiles {

    pub fn get_download(&self) -> String{
        self.downloads[0].clone()
    }



}

