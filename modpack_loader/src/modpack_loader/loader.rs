#![allow(dead_code)]
use std::{fs, time::Duration};
use std::ops::Index;

use reqwest::Response;
use serde_json::{Error};
use tokio::task::{self, JoinHandle};
use tokio::time;

use super::modpack_struct::*;

#[cfg(debug_assertions)]
use std::time::Instant;

enum STATES {
    GOOD,
    FAIL,
    MAGIC   // TODO RENAME THIS SHIT
}

pub struct ModPackDownloader{
    pack: Option<ModPack>,
    state: STATES,
    path: String,
}



impl ModPackDownloader {
    pub fn new() -> ModPackDownloader{
        ModPackDownloader{
            pack: None,
            state: STATES::MAGIC,
            path: "".to_string()
        }
    }

    pub fn load_pack(&mut self, pack_path: String){
        let content = fs::read_to_string(pack_path).unwrap();
        self.pack = { 
            let parsed: Result<Option<ModPack>, Error> = serde_json::from_str(content.as_str()); 
            match parsed {
                Err(e) => {
                    println!("This error happened while parsing: {}\n No modpack will be downloaded", e);
                    None
                }
                Ok(value) => {
                    match value{
                        Some(pack) => Some(pack),
                        None => {None}
                    }
                }
            }
        }
    }

    pub fn set_path(&mut self, _path: String) {
        self.path = _path;
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let n_mods = (&self.pack).as_ref().unwrap().mods().len();
        let mut responses: Vec<JoinHandle<Response>> = Vec::new();
        let mut not_done_mods: Vec<usize> = Vec::from_iter(0..n_mods);
        let minecraft_mods = { 
            let c_pack = self.pack.clone().unwrap();
            c_pack.mods().clone()
        };

        
        for minecraft_mod in &minecraft_mods{
            let link = minecraft_mod.get_file();
            let a_func = async{
                let cliente = reqwest::Client::new();
                cliente.get(link).send().await.unwrap()
            };
            let task = task::spawn(a_func);
            responses.push(task);
        }

        loop {
            for i in not_done_mods.clone(){
                
                let sleep = time::sleep(Duration::from_millis(70));
                tokio::pin!(sleep);

                #[cfg(debug_assertions)]
                println!("Trying task {}\n----------------------", i);
                
                #[cfg(debug_assertions)]
                let start = Instant::now();
                
                tokio::select! {
                    _ = &mut sleep =>  {
                        #[cfg(debug_assertions)]
                        println!("Task {} not ready yet\n", i);
                        continue;
                    }

                    res = &mut responses[i] => {
                        let response = res.unwrap();
                        let full_path = self.path.clone() + minecraft_mods.index(i).get_file_name().as_ref();
                        tokio::fs::write(full_path,
                                         response.bytes().await?).await?;
                        not_done_mods.retain(|&x| x != i);
                    }

                    else => {
                        break;
                    }
                }

                #[cfg(debug_assertions)]{
                    print!("Time passed {:<3}ms \t", start.elapsed().as_millis())
                }
                println!("{}/{}\n", n_mods - &not_done_mods.len(), n_mods);
            }
            if not_done_mods.is_empty() {break;}
        }
        #[cfg(debug_assertions)]{
            println!("/-------------------------------------------\\");
            println!("|##############    FINISH    ################|");
            println!("\\-------------------------------------------/\n\n\n");
        }
        Ok(())
    }
}