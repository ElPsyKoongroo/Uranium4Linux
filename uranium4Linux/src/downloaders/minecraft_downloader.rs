use crate::{
    checker::{check, check_panic, dlog, log, olog},
    code_functions::N_THREADS,
    variables::constants::CANT_CREATE_DIR,
};
use bytes::Bytes;
use mine_data_strutcs::minecraft::{self, MinecraftInstance, MinecraftInstances, Resources};
use once_cell::sync::Lazy;
use reqwest;
use sha1::Digest;
use std::{
    error::Error,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    io::{stdout, AsyncWriteExt},
    task::JoinSet,
};

const ASSESTS_PATH: &str = "assets/";
const INSTANCES_LIST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

static OPEN_OPTIONS: Lazy<tokio::fs::OpenOptions> =
    Lazy::new(|| tokio::fs::OpenOptions::new().write(true).clone());

#[derive(Debug)]
enum SourceState {
    Good { name: String, content: Bytes },
    Bad { name: String },
}

impl SourceState {
    pub fn get_name(&self) -> &str {
        match self {
            SourceState::Good { name, content: _ } => name,
            SourceState::Bad { name } => name,
        }
    }

    pub fn get_bytes(&self) -> Option<&Bytes> {
        match self {
            SourceState::Good { name: _, content } => Some(content),
            SourceState::Bad { name: _ } => None,
        }
    }

    pub fn get_path(&self) -> String {
        let name = match self {
            SourceState::Good { name, content: _ } => name,
            SourceState::Bad { name } => name,
        };
        format!("{}objects/{}/{}", ASSESTS_PATH, &name[..2], name)
    }

    pub fn set_bad(&mut self) {
        *self = match self {
            SourceState::Good { name, content: _ } => SourceState::Bad {
                name: std::mem::take(name),
            },
            SourceState::Bad { name } => SourceState::Bad {
                name: std::mem::take(name),
            },
        };
    }
}

struct MinecraftDownloader {
    test: Vec<SourceState>,
    sources: Resources,
    requester: reqwest::Client,
    chunk_size: usize,
    path: Arc<PathBuf>,
}

impl MinecraftDownloader {
    pub fn new_with_reqwester(
        sources: Resources,
        chunk_size: usize,
        requester: reqwest::Client,
        path: Arc<PathBuf>,
    ) -> MinecraftDownloader {
        MinecraftDownloader {
            test: Vec::new(),
            sources,
            requester,
            chunk_size,
            path,
        }
    }

    pub async fn start(mut self) {
        let start = tokio::time::Instant::now();
        self.download_resources().await.ok();
        let end = tokio::time::Instant::now();
        olog(format!(
            "Download && Write time: {}",
            end.duration_since(start).as_millis()
        ));
    }

    pub fn get_names(&self) -> Vec<&str> {
        self.sources
            .objects
            .values()
            .map(|v| v.hash.as_str())
            .collect()
    }

    pub async fn download_resources(&mut self) -> Result<(), Box<dyn Error>> {
        let mut data: Vec<&minecraft::ObjectData> = self.sources.objects.values().collect();
        let range: Vec<usize> = (0..self.sources.objects.values().len()).collect();
        data.sort_by(|a, b| b.size.cmp(&a.size));

        let mut join_set = JoinSet::new();
        let mut writters = JoinSet::new();

        #[cfg(feature = "console_output")]
        let mut i = 0;

        for indexs in range.chunks(self.chunk_size) {
            let mut sources = Vec::with_capacity(self.chunk_size);
            for i in indexs {
                let rc = self.requester.clone();
                let url = data[*i].get_link();
                let hash = data[*i].hash.clone();
                join_set.spawn(async move {
                    SourceState::Good {
                        name: hash,
                        content: rc
                            .get(&url)
                            .send()
                            .await
                            .unwrap()
                            .bytes()
                            .await
                            .unwrap_or_default(),
                    }
                });
            }

            while let Some(source) = join_set.join_next().await {
                sources.push(source?);
            }
            writters.spawn(MinecraftDownloader::write_files(
                sources,
                Arc::clone(&self.path),
            ));

            #[cfg(feature = "console_output")]
            {
                i += indexs.len();
                print!(
                    "\r{:.2}%",
                    (i * 100) as f64 / self.sources.objects.len() as f64
                );
                stdout().flush();
            }
        }

        let mut bad_files = Vec::new();
        while let Some(w) = writters.join_next().await {
            bad_files.append(&mut w.unwrap());
        }
        Ok(())
    }

    async fn write_files(sources: Vec<SourceState>, dest: Arc<PathBuf>) -> Vec<SourceState> {
        let mut results = Vec::with_capacity(sources.len() / 2);
        for mut source in sources {
            let path = dest.join(source.get_path());
            let mut file = OPEN_OPTIONS.open(&path).await.unwrap();
            let Some(bytes) = source.get_bytes() else {continue};
            //olog(format!("Writting {:?}", path));
            match MinecraftDownloader::check_file(bytes, source.get_name()) {
                Ok(_) => {
                    check(
                        file.write_all(bytes).await,
                        true,
                        "minecraft_downloader; Fail to write resource",
                    )
                    .ok();
                }
                Err(_) => {
                    log(format!("Bad file {}", source.get_name()));
                    source.set_bad();
                    results.push(source);
                }
            }
        }
        results
    }

    fn check_file<T: AsRef<[u8]>>(bytes: T, hash: &str) -> Result<(), hex::FromHexError> {
        let mut hasher = sha1::Sha1::new();
        hasher.update(bytes.as_ref());
        let file_hash = hasher.finalize().to_vec();
        if file_hash != hex::decode(hash)? {
            dlog(format!("Error while checking {:?}, wrong hash", hash));
            return Err(hex::FromHexError::InvalidStringLength);
        }
        Ok(())
    }

    async fn make_files(&self, path: Arc<PathBuf>) -> Result<(), std::io::Error> {
        for file in self.get_names() {
            let path = path.join(ASSESTS_PATH.to_owned() + "objects/" + &file[..2] + "/" + file);
            if tokio::fs::File::create(&path).await.is_err() {
                std::fs::create_dir_all(path.parent().unwrap()).expect(CANT_CREATE_DIR);
                std::fs::File::create(path)?;
            }
        }
        log("Ficheros creados!");
        Ok(())
    }
}

/*

   MINECRAFT INSTANCES VERSIONS/LIST ?

*/

pub async fn print_instances() -> Result<(), reqwest::Error> {
    let requester = reqwest::Client::new();
    let instances = list_instances(&requester).await?;
    instances
        .get_versions_raw()
        .iter()
        .for_each(|t| println!("{}", t.get_id_raw()));

    Ok(())
}

pub async fn list_instances(
    requester: &reqwest::Client,
) -> Result<MinecraftInstances, reqwest::Error> {
    let instances = requester
        .get(INSTANCES_LIST)
        .send()
        .await?
        .json::<minecraft::MinecraftInstances>()
        .await?;

    Ok(instances)
}

/*

        DOWNLOAD MINECRAFT RESOURCES CODE SECTION

*/

async fn get_sourcers(
    requester: &reqwest::Client,
    assets_url: &str,
    destination_path: &Path,
) -> Result<Resources, reqwest::Error> {
    let resources = requester
        .get(assets_url)
        .send()
        .await?
        .json::<minecraft::Resources>()
        .await?;

    check_panic(
        tokio::fs::create_dir_all(destination_path.join("assets/indexes")).await,
        true,
        CANT_CREATE_DIR,
    );

    check_panic(
        tokio::fs::create_dir_all(destination_path.join("assets/objects")).await,
        true,
        CANT_CREATE_DIR,
    );

    Ok(resources)
}

async fn create_indexes(
    resources: &Resources,
    destination_path: &Path,
    assets_url: &str,
) -> Result<(), std::io::Error> {
    let indexes_path = destination_path
        .join("assets/indexes/")
        .join(&assets_url[assets_url.rfind('/').unwrap_or_default() + 1..]);
    let mut indexes = tokio::fs::File::create(indexes_path).await?;

    indexes
        .write_all(
            serde_json::to_string(resources)
                .unwrap_or_default()
                .as_bytes(),
        )
        .await?;

    Ok(())
}

pub async fn download_sourcers(
    assets_url: &str,
    requester: &reqwest::Client,
    destination_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let resources = get_sourcers(requester, assets_url, &destination_path).await?;
    create_indexes(&resources, &destination_path, assets_url).await?;
    let arc_path = Arc::new(destination_path);
    let mc_downloader = MinecraftDownloader::new_with_reqwester(
        resources,
        N_THREADS(),
        requester.clone(),
        arc_path.clone(),
    );
    mc_downloader.make_files(arc_path.clone()).await?;
    mc_downloader.start().await;
    Ok(())
}

pub async fn donwload_minecraft(
    instance: &str,
    destination_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let requester = reqwest::Client::new();
    let intances = list_instances(&requester).await?;
    let instance_url = intances.get_instance_url(instance).unwrap();

    let instance: MinecraftInstance = requester.get(instance_url).send().await?.json().await?;

    download_sourcers(&instance.assest_index.url, &requester, destination_path).await?;

    //TODO!
    // 1.- Learn how the .minecraft folders works
    // 2.- Download the minecraft libraries in the correspondientes folders
    // 3.- Download only the necessary librarias bcs somes are for Windows and somes are for Linux
    // 4.- Try to log with Microsoft account
    // 5.- Try to launch minecraft

    Ok(())
}
