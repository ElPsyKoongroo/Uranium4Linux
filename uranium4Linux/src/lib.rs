#![allow(dead_code)]


mod modpack_downloader;
mod easy_input;
mod hashes;
mod variables;
mod zipper;
mod checker;
mod modpack_maker;
mod code_functions;

use serde_json;
use serde::Deserialize;

use mine_data_strutcs::url_maker::maker;
use requester::requester::request_maker::Requester;
use modpack_downloader::curse_downloader::curse_modpack_downloader;

pub enum RequestType{
    Search,
    Modpack,
    Project,
    Version
}


pub async fn requester(request_type: RequestType, limit: u32, offset: u32, id: String) -> String{
    let url;
    match request_type{
        RequestType::Search  => url = maker::ModRinth::search_for(limit, offset),
        RequestType::Modpack => url = maker::ModRinth::modpacks(),
        RequestType::Project => url = maker::ModRinth::mod_versions_by_id(&id),
        RequestType::Version => url = maker::ModRinth::mod_version_by_id(&id),
    }
    let requester = Requester::new();
    let response = requester.get(url)
        .await
        .unwrap();
    
    let text = response.text().await.unwrap(); 

    text
}

extern crate libc;
use libc::c_char;
use core::slice;
use std::ffi::CString;

#[no_mangle] 
pub extern "C" fn requester_c(request_type: u32 , limit: u32, offset: u32, id: *const u8) -> *const c_char{
    let url;
    unsafe{
        let slice = slice::from_raw_parts(id, 9 as usize);
        let id = CString::from_vec_with_nul_unchecked(slice.to_owned());
        let id = id.to_str().unwrap();

        match request_type{
            0 => url = maker::ModRinth::search_for(limit, offset),
            1 => url = maker::ModRinth::modpacks(),
            2 => url = maker::ModRinth::mod_versions_by_id(id),
            3 => url = maker::ModRinth::mod_version_by_id(id),
            _ => panic!("No se encontro coincidencias")

        }
    }
    let requester = Requester::new();
    
    let handle = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
     
    let response = requester.get(url);
    let a = handle.block_on(response).unwrap();
    let text = a.text();
    let text = handle.block_on(text).unwrap();
    let c_string = CString::new(text).unwrap();
    c_string.into_raw()
}

fn c_string_to_owned(chars: *const u8, lenth: usize) -> String {
    let slice = unsafe{slice::from_raw_parts(chars, lenth)};
    let c_string = unsafe{CString::from_vec_with_nul_unchecked(slice.to_owned())};
    c_string.to_str().unwrap().to_owned()
}

#[no_mangle]
pub extern "C" fn download_curse_pack(path: *const u8, lenth: usize, n_threads: usize){
    let path    = c_string_to_owned(path, lenth);
    let handle  = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

    handle.block_on(curse_modpack_downloader(&path, "./mods/", n_threads));
}

pub fn converter<'a, T: Deserialize<'a>>(text: &'a String) -> T {
    serde_json::from_str(&text).unwrap()
}

