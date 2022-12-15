use std::io::Write;
use bytes::Bytes;

use mine_data_strutcs::minecraft::{self, ObjectData};
use requester::{async_pool::AsyncPool, mod_searcher::search_by_url};
use reqwest;

use crate::code_functions::N_THREADS;

const ASSESTS_PATH: &str = "assets/";
const PLACE_HOLDER: &str =
    "https://piston-meta.mojang.com/v1/packages/c492375ded5da34b646b8c5c0842a0028bc69cec/2.json";

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
    let thread = std::thread::spawn(move || make_files(&names));

    let names: Vec<String> = resources
        .objects
        .files
        .values()
        .map(|v| v.hash.clone())
        .collect();

    let total_size = resources.objects
        .files
        .values()
        .fold(0, |acc, i| acc + i.size.clone());
    println!("Total size: {}", total_size);
    let objects: Vec<ObjectData> = resources.objects.files.values().cloned().collect();
    let mut data = download_resources(resources, &requester).await;


    thread.join().unwrap();
    write_files(&mut data, &names).await;
    
    let wrong_files = check_files(&objects);
    if wrong_files.is_empty() {
        println!("No hay fallos");
        return Ok(())
    }
    for wrong_file in wrong_files {
        println!("Wrong: {}", objects[wrong_file].hash);
    }
    Ok(())
}

pub async fn download_resources(
    resources: minecraft::Resources,
    requester: &reqwest::Client,
) -> Vec<Bytes> {
    let (_names_vec, data): (Vec<String>, Vec<minecraft::ObjectData>) = resources
        .objects
        .files
        .into_iter()
        .map(|(_, b)| (b.hash.clone(), b))
        .unzip();

    let chunk_size = N_THREADS();
    let mut bytes = Vec::with_capacity(3407);
    for files in data.chunks(chunk_size) {
        let mut pool = AsyncPool::new();
        let mut tasks = Vec::with_capacity(chunk_size);

        files
            .iter()
            .for_each(|file| tasks.push(search_by_url(requester, &file.get_link())));

        pool.push_request_vec(tasks);

        pool.start().await;

        /*
        responses.push(
            pool.get_done_request()
                .into_iter()
                .filter_map(|res|
                    match res {
                       Ok(response) => Some(response),
                       Err(error) => {
                           println!("{}", error);
                           None
                       }
                    }
                )
                .collect::<Vec<reqwest::Response>>(),
        );
        */
        push_data(
            pool.get_done_request()
                .into_iter()
                .filter_map(|res| match res {
                    Ok(response) => Some(response),
                    Err(error) => {
                        println!("{}", error);
                        None
                    }
                })
                .collect::<Vec<reqwest::Response>>(),
            chunk_size,
            &mut bytes,
        )
        .await
    }

    // MAGIC !
    bytes
}

async fn push_data(responses: Vec<reqwest::Response>, chunk_size: usize, bytes: &mut Vec<Bytes>) {
    let mut pool = AsyncPool::new();
    let mut tasks = Vec::with_capacity(chunk_size);
    for response in responses {
        tasks.push(tokio::task::spawn(response.bytes()));
    }
    pool.push_request_vec(tasks);
    pool.start().await;
    let mut temp: Vec<Bytes> = pool
        .get_done_request()
        .into_iter()
        .filter_map(|t| match t {
            Ok(e) => Some(e),
            Err(error) => {
                println!("{}", error);
                None
            }
        })
        .collect();
    println!("{}", temp.len());
    bytes.append(&mut temp);
}

async fn write_files(data: &mut [Bytes], names: &[String]) {
    if data.len() != names.len() {
        println!("{} -- {}", data.len(), names.len());
        panic!("Hay algo raro");
    }

    let open_options = std::fs::OpenOptions::new().write(true).to_owned();
    for (file_bytes, name) in data.into_iter().zip(names.iter()) {
        let path = ASSESTS_PATH.to_owned() + "objects/" + &name[..2] + "/" + &name;
        println!("{}", path);
        let mut file = std::io::BufWriter::new(open_options.open(&path).unwrap());
        match file.write_all(file_bytes) {
            Ok(_) => println!("Escrito {}", path),
            Err(_) => println!("Error al escribir {}", path),
        }
    }
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


fn check_files(files: &[ObjectData]) -> Vec<usize> {
    use sha1::Digest;
    use std::io::Read;
    let mut not_ok = Vec::new();
    for i in 0..files.len() {
        let mut hasher = sha1::Sha1::new();
        let path = ASSESTS_PATH.to_owned() + "objects/" + &files[i].hash[..2] + "/" + &files[i].hash;
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_e) => {
                not_ok.push(i);
                continue;
            }
        };
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        hasher.update(&bytes);
        let file_hash = hasher.finalize().to_vec();
        if file_hash != hex::decode(files[i].hash.clone()).unwrap() {
            println!("{}", &files[i].hash);
            not_ok.push(i);
        }
    }
    not_ok
}

