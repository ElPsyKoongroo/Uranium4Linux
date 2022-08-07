#![allow(non_snake_case)]
mod checker;
mod code_functions;
mod easy_input;
mod modpack_downloader;
mod modpack_maker;
mod variables;
mod zipper;
mod hashes;

use code_functions::{update, fix_path};
use modpack_maker::maker::make_modpack;
use zip::result::ZipError;
use std::env;
use variables::constants::*;
use zipper::pack_unzipper::unzip_pack;
use modpack_downloader::curse_downloader::curse_modpack_downloader;

#[tokio::main]
async fn main() -> Result<(), ZipError> {
    let args: Vec<String> = env::args().collect();
    let mut n_threads  = 16; // ONG ONG, MAGIC !
    let mut curse_pack = false;
    let mut file_path = "".to_owned();
    let mut destination_path = "".to_owned();
        

    // Get the number of threads that would be executed
    match args.iter().position(|f| f == "-t") {
        Some(index) => n_threads = args[index+1].parse().unwrap_or(0),
        None => {}
    }

    // This is only going to be executed once while no other function
    // is reading the value so it's actually safe
    unsafe{N_THREADS = n_threads};

    // Get the file path
    match args.iter().position(|f| f == "-f") {
        Some(index) => file_path = args[index+1].clone(),
        None => {}
    }
    

    // Get the destination path 
    match args.iter().position(|f| f == "-r") {
         Some(index) => destination_path = args[index+1].clone(),
         None => {}
    }


    // If the modpack is a curse modpack True      
    match args.iter().position(|f| f == "-c") {
        Some(_) => curse_pack = true,
        None => {}
    }


    destination_path = fix_path(&destination_path);

    match args[1].as_str() {
        "-d" => {
            if !curse_pack {
                unzip_pack(&file_path, &destination_path, n_threads).await.unwrap();
            } else {
                curse_modpack_downloader(&file_path, &destination_path, n_threads).await;
            }
        },
        "-u" => update(args[2].as_str()).await,
        "-m" => make_modpack(&file_path, n_threads).await,
        "-h" => println!("{}", HELP),
        _    => println!("{}", "Invalid arguments")
    }
    Ok(())
}
