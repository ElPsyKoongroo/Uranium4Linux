use std::{io::{stdout, Write}, marker::PhantomData, sync::{Arc, Mutex}};
use std::collections::HashMap;

use bytes::Bytes;
use rayon::{iter::*, slice::ParallelSliceMut};
use reqwest;
use tokio::io::AsyncWriteExt;

use mine_data_strutcs::minecraft;
use requester::{
    async_pool::AsyncPool,
    mod_searcher::{search_by_url, search_by_url_owned},
};

use crate::checker::{check, log};
use crate::code_functions::N_THREADS;

const ASSESTS_PATH: &str = "assets/";
const PLACE_HOLDER: &str =
    "https://piston-meta.mojang.com/v1/packages/c492375ded5da34b646b8c5c0842a0028bc69cec/2.json";

struct MinecraftDownloader<'a> {
    bytes: Vec<Bytes>,
    names: Vec<String>,
    requester: reqwest::Client,
    chunk_size: usize,
    _a: PhantomData<&'a str>,
}

impl<'a> MinecraftDownloader<'a> {
    pub fn new(names: &Vec<String>, chunk_size: usize) -> MinecraftDownloader {
        MinecraftDownloader {
            bytes: Vec::with_capacity(names.len()),
            names: names.clone().to_owned(),
            requester: reqwest::Client::new(),
            chunk_size,
            _a: PhantomData,
        }
    }

    pub async fn start(mut self, resources: minecraft::Resources) {
        let start = tokio::time::Instant::now();

        self.download_resources(resources).await;

        let end = tokio::time::Instant::now();
        println!("Download time: {}", end.duration_since(start).as_millis());

        self.write_files().await;
        self.check_files();
    }

    pub async fn download_resources(&mut self, resources: minecraft::Resources) {
        let mut data: Vec<&minecraft::ObjectData> =
            resources.objects.files.values().map(|b| b).collect();

        data.sort_by_key(|f| f.size);

        let mut i = 0;
        // let mut pool = AsyncPool::new();

        for files in data.chunks(self.chunk_size) {
            let mut join_set = tokio::task::JoinSet::new();
            let mut tasks = Vec::with_capacity(self.chunk_size);

            files.iter().enumerate().for_each(|(i, file)| {
                let rc = self.requester.clone();
                let url = file.get_link();
                tasks.push(join_set.spawn(
                    async move {
                        (rc.get(&url)
                            .send().await, i)
                    }))

            });
            let mut responses: HashMap<usize, reqwest::Response> = HashMap::new();
            for handle in tasks {
                let (result, index) = join_set.join_next().await.unwrap().unwrap();
                responses.insert(index, result.unwrap());
            }

            let ordered_responses: Vec<reqwest::Response> = (0..responses.len()).map(|i| responses.remove(&i).unwrap()).collect();

            // pool.push_request_vec(tasks);
            // pool.start().await;

            #[cfg(feature = "console_output")]
            {
                i += files.len();
                print!("\r{:.2}%         ", (i * 100) as f64 / 3407.0);
                stdout().flush().unwrap();
            }

            self.push_data(
                ordered_responses
                // pool.get_done_request()
                //     .into_iter()
                //     .filter_map(|res| match res {
                //         Ok(response) => Some(response),
                //         Err(error) => {
                //             println!("{}", error);
                //             None
                //         }
                //     })
                //     .collect::<Vec<reqwest::Response>>(),
            )
            .await;
            // pool.clear();
        }
    }

    async fn push_data(&mut self, responses: Vec<reqwest::Response>) {
        let mut pool = AsyncPool::new();
        let mut tasks = Vec::with_capacity(self.chunk_size);

        // for response in responses {
        //     self.bytes.push(response.bytes().await.unwrap())
        // }

        for response in responses {
            tasks.push(tokio::task::spawn(response.bytes()));
        }

        pool.push_request_vec(tasks);
        pool.start().await;
        pool.get_done_request()
            .into_iter()
            .filter_map(|t| match t {
                Ok(e) => Some(e),
                Err(error) => {
                    println!("{}", error);
                    None
                }
            })
            .for_each(|b| self.bytes.push(b));
    }

    async fn write_files(&mut self) {
        if self.bytes.len() != self.names.len() {
            log(format!("{} -- {}", self.bytes.len(), self.names.len()));
            panic!("Hay algo raro");
        }

        let open_options = tokio::fs::OpenOptions::new().write(true).to_owned();
        for (file_bytes, name) in self.bytes.iter().zip(self.names.iter()) {
            let path = ASSESTS_PATH.to_owned() + "objects/" + &name[..2] + "/" + &name;
            let mut file = open_options.open(path).await.unwrap();
            //let mut file = std::io::BufWriter::new(open_options.open(&path).unwrap());

            check(
                file.write_all(file_bytes).await,
                true,
                "minecraft_downloader; Fail to write resource",
            )
            .ok();
        }
    }

    fn check_files(&self) {
        use sha1::Digest;
        use std::io::Read;
        let not_ok: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new())) ;

        (0..self.names.len()).into_par_iter().for_each(|i| {
            let mut hasher = sha1::Sha1::new();
            let path =
                ASSESTS_PATH.to_owned() + "objects/" + &self.names[i][..2] + "/" + &self.names[i];
            let mut file = match std::fs::File::open(path) {
                Ok(file) => file,
                Err(_e) => {
                    let mut guard = not_ok.lock().unwrap();
                    guard.push(i);
                    return ;
                }
            };

            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).unwrap();
            hasher.update(&bytes);
            let file_hash = hasher.finalize().to_vec();
            if file_hash != hex::decode(self.names[i].clone()).unwrap() {
                let mut guard = not_ok.lock().unwrap();
                guard.push(i);
            }}
        );

        #[cfg(debug_assertions)]
        {
            log(format!("Checking complete with {} errors", not_ok.lock().unwrap().len()));
        }
    }
}

pub async fn donwload_minecraft(destionation_path: &str) -> Result<(), reqwest::Error> {
    std::fs::create_dir("assets/indexes").ok();
    std::fs::create_dir("assets/objects").ok();

    let requester = reqwest::Client::new();
    let resources = requester
        .get(PLACE_HOLDER)
        .send()
        .await?
        .json::<minecraft::Resources>()
        .await?;

    let names: Vec<String> = resources
        .objects
        .files
        .values()
        .map(|v| v.hash.clone())
        .collect();

    let mc_downloader = MinecraftDownloader::new(&names, N_THREADS());
    make_files(&names);
    mc_downloader.start(resources).await;
    Ok(())
}

fn make_files(files: &[String]) {
    for file in files {
        let path = ASSESTS_PATH.to_owned() + "objects/" + &file[..2] + "/" + &file;
        let _file = match std::fs::File::create(&path) {
            Ok(e) => e,
            Err(_) => {
                std::fs::create_dir_all(ASSESTS_PATH.to_owned() + "objects/" + &file[..2] + "/")
                    .expect("No se pudo crear el directorio");
                std::fs::File::create(path).unwrap()
            }
        };
    }
    println!("Ficheros creados!");
}
