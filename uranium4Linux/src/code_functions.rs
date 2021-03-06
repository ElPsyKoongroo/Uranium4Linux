use crate::checker::check;
use crate::easy_input::input;


use hex::ToHex;
use mine_data_strutcs::minecraft_mod::*;
use mine_data_strutcs::url_maker::maker;


use requester::requester::request_maker::*;
use sha1::{Digest, Sha1};
use std::fs::{self, read_dir};

use std::io::{Error, ErrorKind, Read};
use std::ops::{Add};
use std::path::Path;


pub enum CODES {
    Exit,
    ModSelected,
    PageSelected,
    SetPath,
    ParseError,
    MakeModPack,
}

#[allow(dead_code)]
pub struct Properties {
    limit: u32,
    page: u32,
    offset: u32,
    selected_mod: usize,
    path: String,
}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            limit: 20,
            page: 0,
            offset: 0,
            selected_mod: 0,
            path: String::from("./"),
        }
    }

    pub fn get_limit(&self) -> u32 {
        self.limit
    }

    pub fn get_page(&self) -> u32 {
        self.page
    }

    #[allow(dead_code)]
    pub fn get_offset(&self) -> u32 {
        self.offset
    }

    pub fn get_selected_mod(&self) -> usize {
        self.selected_mod
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    #[allow(dead_code)]
    pub fn set_limit(&mut self, limit: u32) {
        self.limit = limit;
    }

    pub fn set_page(&mut self, page: u32) {
        self.page = page;
    }

    #[allow(dead_code)]
    pub fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    pub fn set_selected_mod(&mut self, selected_mod: usize) {
        self.selected_mod = selected_mod;
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub async fn download_mod(
    minecraft_mod_versions: &RinthVersions,
    requester: &Requester,
    path: &String,
) -> Result<i32, Box<dyn std::error::Error>> {
    let index: i32 = input("Select the desiere version: ", -1);

    match index.is_negative() {
        false => {
            let response = requester
                .get(
                    minecraft_mod_versions
                        .get_version(index as usize)
                        .get_file_url(),
                )
                .await?;
            let content = response.bytes().await?;
            let path = path.clone().add(
                minecraft_mod_versions
                    .get_version(index as usize)
                    .get_file_name()
                    .as_str(),
            );
            tokio::fs::write(path, content).await?;

            Ok(index)
        }

        true => Err(Box::new(Error::new(ErrorKind::Other, "Bad input!"))),
    }
}

pub fn exits_path(p: &Path) -> bool {
    match Path::new(p).is_dir() {
        true => return true,

        false => {
            eprintln!("This is not a valid directory!!");
            false
        }
    }
}

pub fn set_path() -> String {
    let temp = input("New path: ", String::from("./"));
    let path = temp.as_str();

    match exits_path(Path::new(path)) {
        true => return path.to_string(),
        false => String::from("./")
    }
}

fn get_sha1_from_file(file_path: &String) -> String {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path).unwrap();
    let metadata = fs::metadata(file_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    hasher.update(buffer);
    let temp = hasher.finalize().to_vec();
    let hash = temp.encode_hex::<String>();
    hash
}

pub fn get_mods(mods_path: &Path) -> Option<Vec<(String, String)>> {
    let mut names: Vec<(String, String)> = Vec::new();
    let mods;

    if !mods_path.is_dir() {return None;}
    
    match read_dir(mods_path) {
        Ok(e) => mods = e,
        Err(error) => {
            eprintln!("Error reading the directore: {}", error);
            return None
        }
    }

    for mmod in mods {
        get_sha(mods_path, mmod.unwrap(), &mut names);
    }

    Some(names)
}

fn get_sha(path: &Path, mod_dir: fs::DirEntry, names_vec: &mut Vec<(String, String)>) {
    let file_name = mod_dir.file_name().into_string().unwrap();
    let file_path = { path.join(&file_name).to_str().unwrap().to_string() };
    let hash = get_sha1_from_file(&file_path);
    names_vec.push((hash, file_name));
}

pub fn one_input(input: String) -> CODES {
    match input.as_str() {
        "path" => CODES::SetPath,
        "make" => CODES::MakeModPack,
        "exit" => CODES::Exit,
        _ => CODES::ParseError,
    }
}

pub fn two_inputs(opt: String, value: &str, properties: &mut Properties) -> CODES {
    match opt.as_str() {
        "mod" => {
            properties.set_selected_mod(value.parse::<usize>().unwrap());
            return CODES::ModSelected;
        }
        "page" => {
            properties.set_page(value.parse::<u32>().unwrap());
            return CODES::PageSelected;
        }

        _ => return CODES::ParseError,
    }
}

pub async fn search_mods_for_modpack(requester: &mut Requester, hash_filename: Vec<(String, String)>, responses: &mut RinthVersions) {
    for item in hash_filename {
        let response = {
            let request = requester.get(maker::ModRinth::hash(&item.0)).await.unwrap();
            check(
                request.json::<RinthVersion>().await,
                false,
                true,
                format!("Mod {} was not found !", &item.1).as_str(),
            )
        };
        match response {
            Some(e) => responses.push(e),
            None => {}
        }
    }
}