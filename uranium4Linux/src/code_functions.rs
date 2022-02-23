use std::fs::{read_dir, self};
use std::hash;
use std::ops::{Index, Add};
use std::io::{Error, ErrorKind, Stderr, Read};
use std::path::Path;
use regex::Regex;
use tokio::fs::File;
use requester::requester::load_headers::*;
use requester::requester::request_maker::*;
use sha1::{Sha1, Digest};
use hex::{FromHex, ToHex};
use minecraft_mod::minecraft_mod::*;
use minecraft_mod::responses::*;

use crate::easy_input::input;

pub enum CODES {
    Exit,
    ModSelected,
    PageSelected,
    SetPath,
    ParseError,
    MakeModPack,
}

#[warn(dead_code)]
pub struct Properties{
    pub limit: u32,
    pub page: u32,
    pub offset: u32,
    pub selected_mod: usize,
    pub path: String
}


pub fn clear_screen(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

}

pub async fn download_mod(minecraft_mod_versions: &RinthVersions, requester: &Requester, path: &String) -> Result<i32, Box<dyn std::error::Error>>{
   
    let index: i32 = input("Select the desiere version: ", -1);
   
    match index.is_negative(){
        false => {
            let response = requester.get(
                minecraft_mod_versions.get_version(index as usize).get_file_url()
            ).await?;
            let content = response.bytes().await?;
            let path = path.clone().add(minecraft_mod_versions.get_version(index as usize).get_file_name().as_str());
            tokio::fs::write(path, content).await?;

            Ok(index)
        }

        true => {
            Err(Box::new(Error::new(ErrorKind::Other, "Bad input!")))
        }
    }
}

pub fn exits_path(p: &Path) -> bool{
    match  Path::new(p).is_dir(){
        true => {
            return true
        }

        false =>{
            println!("This is not a valid directory!!");
            false
        }
    }
}

pub fn set_path() -> String{
    let temp = input("New path: ", String::from("./"));
    let path = temp.as_str();
    

    match  exits_path(Path::new(path)){
        true => {
            return path.to_string()
        }

        false =>{
            println!("This is not a valid directory!!");
            String::from("./")
        }
    }

}


fn get_sha1_from_file(file_path: &String)->String{
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path).unwrap();
    let metadata = fs::metadata(file_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");
    
    hasher.update(buffer);
    let temp = hasher.finalize().to_vec();
    let hash =  temp.encode_hex::<String>();
    hash
}

pub fn get_mods(p: &Path)-> Option<Vec<(String, String)>>{
    let mut names: Vec<(String, String)> = Vec::new();
    let mods;
    match p.is_dir(){
        true => {
            mods = read_dir(p).unwrap();
            for mine_mod in mods {
                match mine_mod {
                    Ok(ok_mod) =>{
                        let file_name = ok_mod.file_name().into_string().unwrap();
                        let file_path = {
                            p.join(&file_name).to_str().unwrap().to_string()
                        };
                        let hash = get_sha1_from_file(&file_path);
                        names.push((hash, file_name));
                    }

                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            Some(names)
        }
        _ => {
            None
        }
    }
}