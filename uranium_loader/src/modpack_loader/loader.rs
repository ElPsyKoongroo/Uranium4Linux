use std::ops::Index;
use std::time::Duration;

use mine_data_strutcs::modpack_mod::Mods;
use mine_data_strutcs::modpack_struct::{ModPack, load_pack};
use reqwest::Response;
use tokio::task::{self, JoinHandle};
use tokio::time;

#[cfg(debug_assertions)]
use std::time::Instant;

#[allow(dead_code)]
pub struct ModPackDownloader {
    pack: Option<ModPack>,
    path: String,
}

async fn request_maker(minecraft_mods: &Vec<Mods>) -> Vec<JoinHandle<Response>> {
    let mut responses = Vec::new();
    for minecraft_mod in minecraft_mods {
        let link = minecraft_mod.get_file();
        let a_func = async {
            let cliente = reqwest::Client::new();
            cliente.get(link).send().await.unwrap()
        };
        let task = task::spawn(a_func);
        responses.push(task);
    }
    responses
}

impl ModPackDownloader {
    pub fn new() -> ModPackDownloader {
        ModPackDownloader {
            pack: None,
            path: "".to_string(),
        }
    }

    pub fn load_pack(&mut self, pack_path: String) {
        self.pack = load_pack(&pack_path);
    }

    pub fn set_path(&mut self, mut _path: String) {
        if !_path.ends_with("/") {
            _path.push('/');
        }
        self.path = _path;
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let n_mods = (&self.pack).as_ref().unwrap().mods().len();
        let not_done_mods: Vec<usize> = Vec::from_iter(0..n_mods);
        let minecraft_mods = {
            let c_pack = self.pack.clone().unwrap();
            c_pack.mods().clone()
        };

        let responses: Vec<JoinHandle<Response>> = request_maker(&minecraft_mods).await;
        
        self.download_v2(not_done_mods, responses, minecraft_mods).await
        
    }

    async fn download_v2(
        &mut self,
        mut not_done_mods: Vec<usize>,
        mut responses: Vec<JoinHandle<Response>>,
        minecraft_mods: Vec<Mods>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(debug_assertions)]
        let start = Instant::now();
        loop {
            let done_mod = self.download_loop(not_done_mods.clone(), &mut responses, &minecraft_mods).await;
            not_done_mods.retain(|&x| x != done_mod);
        
            if not_done_mods.is_empty() {
                break;
            }
        }
        #[cfg(debug_assertions)]
        print!("{:<3}\n", start.elapsed().as_millis());
        
        Ok(())
    }


    async fn download_loop(
        &mut self,
        not_done_mods: Vec<usize>,
        responses: &mut Vec<JoinHandle<Response>>,
        minecraft_mods: &Vec<Mods>,
    ) -> usize{
        for i in not_done_mods.clone() {
            let sleep = time::sleep(Duration::from_millis(50));
            tokio::pin!(sleep);


            tokio::select! {
                _ = &mut sleep =>  {
                    continue;
                }
                
                res = &mut responses[i] => {
                    let web_res = res.unwrap();
                    let full_path = self.path.clone() + minecraft_mods.index(i).get_file_name().as_ref(); 
                    let content = web_res.bytes().await.unwrap();
                    tokio::fs::write(full_path, content).await.unwrap();
                }
                
                else => {
                    break;
                }
            }
            return i;
        }
        0
    }
}
