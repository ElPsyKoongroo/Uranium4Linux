use core::panic;
use std::fs;
use std::ops::Index;

use crate::code_functions::fix_path;
use mine_data_strutcs::modpack_mod::Mods;
use mine_data_strutcs::modpack_struct::{load_pack, ModPack};
use reqwest::Response;
use requester::async_pool;
use tokio::task::{self, JoinHandle};

use std::time::Instant;

#[allow(dead_code)]
pub struct ModPackDownloader {
    pack: Option<ModPack>,
    path: String,
}

async fn request_maker(minecraft_mods: &Vec<Mods>) -> Vec<JoinHandle<Result<Response, reqwest::Error>>> {
    let mut responses: Vec<JoinHandle<Result<Response, reqwest::Error>>> = Vec::new();
    let cliente = reqwest::Client::new();
    for minecraft_mod in minecraft_mods {
        let link = minecraft_mod.get_file();

        let a_func = cliente.get(link).send();

        let task = task::spawn(a_func);
        responses.push(task);
    }
    responses
}

fn writters_maker(path: String, responses: Vec<Response>, minecraft_mods: &Vec<Mods>,) -> Vec<JoinHandle<()>>{
    let mut i = 0;
    let mut writters = Vec::new();
    for response in responses.into_iter(){
        let path_copy = path.clone();
        let mod_name = minecraft_mods.index(i.clone()).get_file_name();
        let task = async move {
            write_mod(
                &path_copy, 
                response,
                &mod_name
            ).await;
        }; 
        writters.push(tokio::spawn(task));
        i += 1;
    }
    writters
}

async fn write_mod(path: &str, res: Response, name: &str){
    let web_res = res;
    let full_path = path.to_owned() + name;
    let content = web_res.bytes().await.unwrap();
    tokio::fs::write(full_path, content).await.unwrap();
 
}

impl ModPackDownloader {
    pub fn new() -> ModPackDownloader {
        ModPackDownloader {
            pack: None,
            path: "".to_string(),
        }
    }

    pub fn load_pack(&mut self, pack_path: &str) {
        self.pack = load_pack(pack_path);
    }

    pub fn set_path(&mut self, mut _path: String) {
        _path = fix_path(&_path).to_owned();
        _path.push_str("mods/");
              

        if !std::path::Path::new(&_path).exists(){
            fs::create_dir(&_path).unwrap();
        }

        self.path = _path;
    }

    pub async fn start<'a>(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let minecraft_mods: &Vec<Mods>;
        let responses: Vec<JoinHandle<Result<Response, _>>>;
        match &self.pack {
            Some(modpack) => {
                minecraft_mods = modpack.mods();
                responses = request_maker(minecraft_mods).await;
            },
            None => panic!("No modpack !")
        }


        // Start the pool request
        let mut pool = async_pool::AsyncPool::new();
        pool.push_request_vec(responses);

        let start = Instant::now();

        pool.start().await;
       
        let end = Instant::now();
        

        println!("{:?}", end.duration_since(start).as_millis());

        let responses = pool
        .get_done_request()
        .into_iter()
        .map(|f| f.unwrap())
        .collect();

        

        // Start the writting pool
        let writters: Vec<JoinHandle<()>> = writters_maker(self.path.clone(), responses, minecraft_mods);
        let mut pool = async_pool::AsyncPool::new();
        pool.push_request_vec(writters);
        pool.start().await;
 
        Ok(())
    } 

}
