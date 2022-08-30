#![allow(non_snake_case)]
mod checker;
mod code_functions;
mod easy_input;
mod hashes;
mod modpack_downloader;
mod modpack_maker;
mod searcher;
mod variables;
mod zipper;

use code_functions::{fix_path, get_bool_element, get_parse_element, update, N_THREADS};
use modpack_downloader::curse_downloader::curse_modpack_downloader;
use modpack_downloader::rinth_downloader::download_rinth_pack;
use modpack_maker::maker::{make_modpack, ModAttributes};
use searcher::rinth::SEARCH_TYPE;
use std::env;
use std::str::FromStr;
use variables::constants::*;
use zip::result::ZipError;
use zipper::pack_unzipper::unzip_pack;

fn request_arg_parser(args: &Vec<String>) -> Option<SEARCH_TYPE> {
    match args.iter().position(|f| f == REQUEST) {
        Some(index) => match args[index + 1].as_str() {
            QUERY => Some(SEARCH_TYPE::QUERRY(args[index + 2].clone())),
            FOR => Some(SEARCH_TYPE::FOR(
                args[index + 2].parse().unwrap(),
                args[index + 3].parse().unwrap(),
            )),
            VERSION => Some(SEARCH_TYPE::VERSION(args[index + 1].clone())),
            VERSIONS => Some(SEARCH_TYPE::VERSIONS(args[index + 1].clone())),
            MOD => Some(SEARCH_TYPE::MOD(args[index + 1].clone())),
            PROJECT => Some(SEARCH_TYPE::PROJECT(args[index + 1].clone())),
            RESOURCEPACKS => Some(SEARCH_TYPE::RESOURCEPACKS(
                args[index + 2].parse().unwrap(),
                args[index + 3].parse().unwrap(),
            )),

            _ => panic!("No request type !"),
        },
        None => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), ZipError> {
    let args: Vec<String> = env::args().collect();
    let curse_pack;
    let rinth_pack;
    let mut file_path;
    let mut destination_path;

    // Get the number of threads that would be executed
    let mut aux = NTHREADS.write().unwrap();
    *aux = get_parse_element(&args, THREADS_FLAG).unwrap_or(DEFAULT_NTHREADS);
    // Get the file path
    file_path = get_parse_element(&args, FILE_FLAG).unwrap_or("".to_owned());
    // Get the destination path
    destination_path = get_parse_element(&args, ROOT_FLAG).unwrap_or("".to_owned());
    // If the modpack is a curse modpack True
    curse_pack = get_bool_element(&args, CURSE_FLAG);
    // If the modpack is a rinth modpack True
    rinth_pack = get_bool_element(&args, RINTH_FLAG);

    destination_path = fix_path(&destination_path);

    match args[1].as_str() {
        DOWNLOAD => {
            if curse_pack {
                curse_modpack_downloader(&file_path, &destination_path).await;
            } else if rinth_pack {
                download_rinth_pack(&file_path, &destination_path, N_THREADS()).await;
            } else {
                unzip_pack(&file_path, &destination_path, N_THREADS())
                    .await
                    .unwrap();
            }
        }
        "-u" => update(args[2].as_str()).await,
        MAKE => make_modpack(&file_path, N_THREADS()).await,
        REQUEST => searcher::rinth::search(request_arg_parser(&args).unwrap()).await,
        HELP => println!("{}", HELP_MSG),
        _ => println!("{}", "Invalid arguments"),
    }
    Ok(())
}
