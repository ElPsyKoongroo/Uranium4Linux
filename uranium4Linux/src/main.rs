#![allow(non_snake_case)]
mod checker;
mod code_functions;
mod easy_input;
mod hashes;
mod modpack_downloader;
mod modpack_maker;
mod variables;
mod zipper;

use code_functions::{fix_path, update};
use modpack_downloader::curse_downloader::curse_modpack_downloader;
use modpack_downloader::rinth_downloader::download_rinth_pack;
use modpack_maker::maker::make_modpack;
use std::env;
use variables::constants::*;
use zip::result::ZipError;
use zipper::pack_unzipper::unzip_pack;

#[tokio::main]
async fn main() -> Result<(), ZipError> {
    let args: Vec<String> = env::args().collect();
    let mut n_threads = 16; // ONG ONG, MAGIC !
    let mut curse_pack = false;
    let mut rinth_pack = false;
    let mut file_path = "".to_owned();
    let mut destination_path = "".to_owned();

    // Get the number of threads that would be executed
    match args.iter().position(|f| f == "-t") {
        Some(index) => n_threads = args[index + 1].parse().unwrap_or(0),
        None => {}
    }

    // Get the file path
    match args.iter().position(|f| f == "-f") {
        Some(index) => file_path = args[index + 1].clone(),
        None => {}
    }

    // Get the destination path
    match args.iter().position(|f| f == "-m") {
        Some(index) => destination_path = args[index + 1].clone(),
        None => {}
    }

    // If the modpack is a curse modpack True
    match args.iter().position(|f| f == "-c") {
        Some(_) => curse_pack = true,
        None => {}
    }

    // If the modpack is a curse modpack True
    match args.iter().position(|f| f == "-r") {
        Some(_) => rinth_pack = true,
        None => {}
    }

    destination_path = fix_path(&destination_path);

    match args[1].as_str() {
        "-d" => {
            if curse_pack {
                curse_modpack_downloader(&file_path, &destination_path, n_threads).await;
            } else if rinth_pack {
                download_rinth_pack(&file_path, &destination_path, n_threads).await;
            } else {
                unzip_pack(&file_path, &destination_path, n_threads)
                    .await
                    .unwrap();
            }
        }
        "-u" => update(args[2].as_str()).await,
        "-m" => make_modpack(&file_path, n_threads).await,
        "-h" => println!("{}", HELP),
        _ => println!("{}", "Invalid arguments"),
    }
    Ok(())
}
