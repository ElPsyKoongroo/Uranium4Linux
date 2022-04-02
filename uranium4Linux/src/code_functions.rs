#![allow(dead_code)]
use crate::easy_input::input;
use hex::{FromHex, ToHex};
use minecraft_mod::minecraft_mod::*;
use minecraft_mod::responses::*;
use regex::Regex;
use requester::requester::load_headers::*;
use requester::requester::request_maker::*;
use sha1::{Digest, Sha1};
use std::fs::{self, read_dir};
use std::hash;
use std::io::{Error, ErrorKind, Read, Stderr};
use std::ops::{Add, Index};
use std::path::Path;
use tokio::fs::File;

pub enum CODES {
    Exit,
    ModSelected,
    PageSelected,
    SetPath,
    ParseError,
    MakeModPack,
}

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

    pub fn get_offset(&self) -> u32 {
        self.offset
    }

    pub fn get_selected_mod(&self) -> usize {
        self.selected_mod
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn set_limit(&mut self, limit: u32) {
        self.limit = limit;
    }

    pub fn set_page(&mut self, page: u32) {
        self.page = page;
    }

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
            println!("This is not a valid directory!!");
            false
        }
    }
}

pub fn set_path() -> String {
    let temp = input("New path: ", String::from("./"));
    let path = temp.as_str();

    match exits_path(Path::new(path)) {
        true => return path.to_string(),

        false => {
            println!("This is not a valid directory!!");
            String::from("./")
        }
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

    if !mods_path.is_dir() {
        return None;
    }

    mods = read_dir(mods_path).unwrap();

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
