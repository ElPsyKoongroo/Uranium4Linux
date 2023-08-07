use crate::variables::constants::CANT_CREATE_DIR;

use fs_extra::dir::create_all;
use mine_data_strutcs::minecraft::{
    self, Lib, Libraries, MinecraftInstance, MinecraftInstances, ObjectData, Resources,
};
use reqwest;
use sha1::Digest;
use std::{
    error::Error,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

//#[cfg(feature = "console_output")]
use std::io::Write;

use log::{warn, error};

use super::gen_downloader;

const DEFAULT_FILE_BUFFER_SIZE: usize = 16384;
const ASSESTS_PATH: &str = "assets/";
const OBJECTS_PATH: &str = "objects";
const INSTANCES_LIST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

use thiserror::Error;

#[derive(Error, Debug)]
enum HashCheckError {
    #[error("Invalid hash")]
    BadHash,
    #[error("Hash doesnt match")]
    HashDoesntMatch,
}

///
/// Returns Ok(()) when the hash matches.
/// Otherwise Err
///
fn check_file<T: AsRef<[u8]>>(bytes: T, hash: &str) -> Result<(), HashCheckError> {
    let mut hasher = sha1::Sha1::new();
    hasher.update(bytes.as_ref());
    let file_hash = hasher.finalize().to_vec();
    if file_hash != hex::decode(hash).map_err(|_| HashCheckError::BadHash)? {
        warn!("Error while checking {:?}, wrong hash", hash);
        return Err(HashCheckError::HashDoesntMatch);
    }
    Ok(())
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

    if tokio::fs::create_dir_all(destination_path.join("assets/indexes")).await.is_err() {
        error!("{CANT_CREATE_DIR}");
    }

    if tokio::fs::create_dir_all(destination_path.join("assets/objects")).await.is_err() {
        error!("{CANT_CREATE_DIR}");
    }

    Ok(resources)
}

async fn create_indexes(
    resources: &Resources,
    destination_path: &Path,
    assets_url: &str,
) -> Result<(), std::io::Error> {
    let indexes_path = destination_path
        .join(ASSESTS_PATH)
        .join("indexes")
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

///   
///   # How this function works:  
///  
///  - Create the corresponding files before downloading the data
///  - Download the data
///
pub async fn download_sourcers(
    assets_url: &str,
    requester: reqwest::Client,
    destination_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let resources = get_sourcers(&requester, assets_url, &destination_path).await?;
    create_indexes(&resources, &destination_path, assets_url).await?;

    let names: Vec<PathBuf> = resources
        .objects
        .values()
        .map(|file| {
            PathBuf::from(
                ASSESTS_PATH.to_owned() + OBJECTS_PATH + &file.hash[..2] + "/" + &file.hash,
            )
        })
        .collect();

    let urls = resources
        .objects
        .values()
        .map(ObjectData::get_link)
        .collect::<Vec<String>>();

    for p in &names {
        create_all(destination_path.join(p).parent().unwrap(), false)
            .expect("Error while creating the sources directories");
    }

    let gen_downloader = gen_downloader::Downloader {
        names,
        requester,
        urls: urls.into(),
        path: destination_path.clone().into(),
    };

    gen_downloader.start().await;

    let n_files = resources.objects.len();
    let mut i = 0;
    for file in resources.objects.values() {
        let file_path = destination_path
            .join(ASSESTS_PATH)
            .join(OBJECTS_PATH)
            .join(file.get_path());

        let mut reader = tokio::io::BufReader::new(
            tokio::fs::File::open(&file_path)
                .await
                .expect("Unable to open the file"),
        );
        let mut buffer = Vec::with_capacity(DEFAULT_FILE_BUFFER_SIZE);
        reader
            .read_to_end(&mut buffer)
            .await
            .expect("Error while reading the file");

        if buffer.len() != file.size && check_file(buffer, &file.hash).is_err() {
            println!("Error in file {}", file_path.to_str().unwrap());
        }
        i += 1;
        print!("\r{}/{}     ", i, n_files);
        let _ = std::io::stdout().lock().flush();
    }
    Ok(())
}

pub async fn download_libraries(
    requester: reqwest::Client,
    destination_path: PathBuf,
    libraries: Libraries,
) {
    let raw_paths = libraries.get_paths();
    let urls = libraries
        .get_ulrs()
        .into_iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .into();

    let good_paths: Vec<PathBuf> = raw_paths
        .iter()
        .map(|p| PathBuf::from("libraries").join(p))
        .collect();

    for p in &good_paths {
        create_all(destination_path.join(p).parent().unwrap(), false)
            .expect("Unable to create Lib's dirs");
    }

    let gen_downloader = gen_downloader::Downloader {
        names: good_paths,
        requester,
        urls,
        path: destination_path.into(),
    };

    gen_downloader.start().await;
}

pub async fn donwload_minecraft(
    instance: &str,
    destination_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let requester = reqwest::Client::new();
    let intances = list_instances(&requester)
        .await
        .expect("Couldnt get minecraft versions");
    let instance_url = intances
        .get_instance_url(instance)
        .unwrap_or_else(|| panic!("Couldnt find {instance} version"));

    let instance: MinecraftInstance = requester.get(instance_url).send().await?.json().await?;

    download_libraries(
        requester.clone(),
        destination_path.clone(),
        instance.libraries,
    )
    .await;

    download_sourcers(
        &instance.assest_index.url,
        requester.clone(),
        destination_path,
    )
    .await?;

    //TODO!
    // Done 1.- Learn how the .minecraft folders works
    // Done 2.- Download the minecraft libraries in the correspondientes folders
    // 3.- Download only the necessary librarias bcs somes are for Windows and somes are for Linux
    // 4.- Try to log with Microsoft account
    // 5.- Try to launch minecraft

    Ok(())
}
