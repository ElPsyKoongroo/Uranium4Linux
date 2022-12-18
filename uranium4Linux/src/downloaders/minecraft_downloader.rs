use std::io::stdout;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use sha1::Digest;
use bytes::Bytes;
use reqwest;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use mine_data_strutcs::minecraft;
use crate::checker::{check, check_default, check_panic, log, olog};
use crate::code_functions::N_THREADS;
use crate::variables::constants::{CANT_CREATE_DIR, DOWNLOAD_ERROR_MSG};

const ASSESTS_PATH: &str = "assets/";
const PLACE_HOLDER: &str =
    "https://piston-meta.mojang.com/v1/packages/c492375ded5da34b646b8c5c0842a0028bc69cec/2.json";

struct MinecraftDownloader<'a> {
    bytes: Vec<Bytes>,
    names: &'a [&'a str],
    requester: reqwest::Client,
    chunk_size: usize,
}

impl<'a> MinecraftDownloader<'a> {
    pub fn new(names: &'a[&'a str], chunk_size: usize) -> MinecraftDownloader {
        MinecraftDownloader {
            bytes: Vec::with_capacity(names.len()),
            names: &names,
            requester: reqwest::Client::new(),
            chunk_size,
        }
    }

    pub async fn start(mut self, resources: &minecraft::Resources) {
        let start = tokio::time::Instant::now();
        self.download_resources(&resources).await;
        let end = tokio::time::Instant::now();
        olog(format!("\nDownload time: {}", end.duration_since(start).as_millis()));
        self.write_files().await;
        self.check_files().await;
    }

    pub async fn download_resources(&mut self, resources: &minecraft::Resources) {
        let mut data: Vec<&minecraft::ObjectData> = resources.objects.files.values().collect();
        data.sort_by(|a, b| b.size.cmp(&a.size));

        let mut i = 0;
        let mut join_set = tokio::task::JoinSet::new();
        let mut responses: HashMap<usize, Bytes> = HashMap::new();

        for files in data.chunks(self.chunk_size) {
            files.iter().enumerate().for_each(|(i, file)| {
                let rc = self.requester.clone();
                let url = file.get_link();
                join_set.spawn(
                    async move {
                        (rc.get(&url).send().await.unwrap().bytes().await, i)
                    });
            });

            while let Some(Ok((result, index))) = join_set.join_next().await {
                responses.insert(index, check_default(result, false, DOWNLOAD_ERROR_MSG));
            }

            #[cfg(feature = "console_output")]
            {
                i += files.len();
                print!("\r{:.2}%", (i * 100) as f64 / 3407.0);
                stdout().flush().unwrap();
            }
            (0..responses.len()).for_each(|i| self.bytes.push(responses.remove(&i).unwrap()));
        }
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
            check(
                file.write_all(&file_bytes).await,
                true,
                "minecraft_downloader; Fail to write resource",
            )
            .ok();
        }
    }

    async fn check_files(&self) {
        let not_ok: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new())) ;
        for indexs in (0..self.names.len()).collect::<Vec<usize>>().chunks(self.chunk_size) {
            let mut join_set = tokio::task::JoinSet::new();
            indexs.into_iter().for_each(|i| {
                let i = i.clone();
                let nk = Arc::clone(&not_ok);
                let path = ASSESTS_PATH.to_owned() + "objects/" + &self.names[i][..2] + "/" + &self.names[i];
                join_set.spawn(async move {MinecraftDownloader::check_file(path, nk, i).await});
            });
            while let Some(_) = join_set.join_next().await {}
        }

        #[cfg(debug_assertions)]
        {
            log(format!("Checking complete with {} errors", not_ok.lock().unwrap().len()));
        }
    }

    async fn check_file(path: String, nk: Arc<Mutex<Vec<usize>>>, i: usize){
        let mut hasher = sha1::Sha1::new();
        let mut file = match tokio::fs::File::open(&path).await {
            Ok(file) => file,
            Err(_e) => {
                let mut guard = nk.lock().unwrap();
                guard.push(i);
                return ;
            }
        };

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).await.unwrap();
        hasher.update(&bytes);
        let file_hash = hasher.finalize().to_vec();
        let hash = &path[(path.rfind('/').unwrap())+1..];
        if file_hash != hex::decode(hash).unwrap() {
            let mut guard = match nk.lock() {
                Ok(g) => g,
                Err(_e) => {return ;}
            };
            guard.push(i);
        }
    }

    fn make_files(files: &[&str]) {
        for file in files {
            let path = ASSESTS_PATH.to_owned() + "objects/" + &file[..2] + "/" + &file;
            let _file = match std::fs::File::create(&path) {
                Ok(e) => e,
                Err(_) => {
                    std::fs::create_dir_all(ASSESTS_PATH.to_owned() + "objects/" + &file[..2] + "/")
                        .expect(CANT_CREATE_DIR);
                    std::fs::File::create(path).unwrap()
                }
            };
        }
        log("Ficheros creados!");
    }
}

pub async fn donwload_minecraft(_destination_path: &str) -> Result<(), reqwest::Error> {
    check_panic(tokio::fs::create_dir_all("assets/indexes").await, false, CANT_CREATE_DIR);
    check_panic(tokio::fs::create_dir_all("assets/objects").await, false, CANT_CREATE_DIR);

    let requester = reqwest::Client::new();
    let resources = requester
        .get(PLACE_HOLDER)
        .send()
        .await?
        .json::<minecraft::Resources>()
        .await?;

    let names: Vec<&str> = resources
        .objects
        .files
        .values()
        .map(|v| v.hash.as_str())
        .collect();

    let mc_downloader = MinecraftDownloader::new(&names, N_THREADS());
    MinecraftDownloader::make_files(&names);
    mc_downloader.start(&resources).await;
    Ok(())
}


