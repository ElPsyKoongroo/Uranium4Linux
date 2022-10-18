use super::functions::get_writters;
use crate::code_functions::fix_path;
use core::panic;
use mine_data_strutcs::uranium_modpack::modpack_mod::Mods;
use mine_data_strutcs::uranium_modpack::modpack_struct::{load_pack, UraniumPack};
use requester::async_pool;
use reqwest::Response;
use std::fs;
use tokio::task::{self, JoinHandle};

#[allow(dead_code)]
pub struct ModPackDownloader {
    pack: Option<UraniumPack>,
    path: String,
    n_threads: usize,
}

fn request_maker(minecraft_mods: &Vec<Mods>) -> Vec<JoinHandle<Result<Response, reqwest::Error>>> {
    let mut responses: Vec<JoinHandle<Result<Response, reqwest::Error>>> =
        Vec::with_capacity(minecraft_mods.len());
    let cliente = reqwest::Client::new();
    for minecraft_mod in minecraft_mods {
        let link = minecraft_mod.get_file();

        let a_func = cliente.get(link).send();

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
            n_threads: 0,
        }
    }

    pub fn new_with_threads(n_threads: usize) -> ModPackDownloader {
        ModPackDownloader {
            pack: None,
            path: "".to_string(),
            n_threads,
        }
    }

    pub fn load_pack(&mut self, pack_path: &str) {
        self.pack = load_pack(pack_path);
    }

    pub fn set_path(&mut self, mut new_path: String) {
        // In case the user enter "/some/random/path" and forgot the last '/'
        new_path = fix_path(&new_path);
        new_path.push_str("mods/");

        if !std::path::Path::new(&new_path).exists() {
            fs::create_dir(&new_path).unwrap();
        }

        self.path = new_path;
    }

    pub async fn start<'a>(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running {} threads!", self.n_threads);
        if self.n_threads == 0 {
            self.unlimited_pool().await
        } else {
            self.limited_pool().await;
            Ok(())
        }
    }

    pub async fn unlimited_pool(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let minecraft_mods: &Vec<Mods>;
        let responses: Vec<JoinHandle<Result<Response, _>>>;
        match &self.pack {
            Some(modpack) => {
                minecraft_mods = modpack.mods();
                responses = request_maker(minecraft_mods);
            }
            None => panic!("No modpack !"),
        }
        self.download_and_write(responses, minecraft_mods)
            .await
            .unwrap();
        Ok(())
    }

    // In case the user want to run Uranium with N threads
    pub async fn limited_pool(&mut self) {
        let modpack_mods = match &self.pack {
            Some(modpack) => modpack.mods(),
            None => panic!("No modpack!"),
        };

        // Chunk the modpack_mods vector into chunks of n_threads elements
        let chunks = modpack_mods
            .chunks(self.n_threads)
            .collect::<Vec<&[Mods]>>();

        for chunk in chunks {
            let vec_chunk = chunk.to_vec();
            let responses = request_maker(&vec_chunk);
            self.download_and_write(responses, &vec_chunk)
                .await
                .unwrap();
        }
    }

    async fn download_and_write(
        &self,
        responses: Vec<JoinHandle<Result<Response, reqwest::Error>>>,
        minecraft_mods: &[Mods],
    ) -> Result<(), std::fmt::Error> {
        // Start the pool request
        let mut pool = async_pool::AsyncPool::new();
        pool.push_request_vec(responses);
        pool.start().await;

        let responses = pool.get_done_request().into_iter().flatten().collect();

        let mod_names: Vec<String> = minecraft_mods
            .iter()
            .map(Mods::get_file_name)
            .collect::<Vec<String>>();

        // Start the writting pool
        let writters: Vec<JoinHandle<()>> =
            get_writters(responses, mod_names, &self.path.clone()).await;
        let mut pool = async_pool::AsyncPool::new();
        pool.push_request_vec(writters);
        pool.start().await;

        Ok(())
    }
}
