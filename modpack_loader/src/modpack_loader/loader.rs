use std::ops::Index;
use std::time::Duration;

use reqwest::Response;
use tokio::task::{self, JoinHandle};
use tokio::time;

use super::modpack_struct::*;

#[cfg(debug_assertions)]
use std::time::Instant;

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
        self.download(not_done_mods, responses, minecraft_mods)
            .await
    }

    async fn download(
        &mut self,
        mut not_done_mods: Vec<usize>,
        mut responses: Vec<JoinHandle<Response>>,
        minecraft_mods: Vec<Mods>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(debug_assertions)]
        let start = Instant::now();
        loop {
            for i in not_done_mods.clone() {
                let sleep = time::sleep(Duration::from_millis(50));
                tokio::pin!(sleep);
    
                #[cfg(debug_assertions)]
                println!("Trying task {}\n----------------------", i);
    
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
                                            response.bytes().await.unwrap()).await?;
                        not_done_mods.retain(|&x| x != i);
                    }

                    else => {
                        break;
                    }
                }
            }
            if not_done_mods.is_empty() {
                break;
            }
        }
        #[cfg(debug_assertions)]
        {
            print!("{:<3}ms has passed !! \t\n", start.elapsed().as_millis());
            println!("/-------------------------------------------\\");
            println!("|##############    FINISH    ################|");
            println!("\\-------------------------------------------/\n\n\n");
        }
        Ok(())
    }

}
