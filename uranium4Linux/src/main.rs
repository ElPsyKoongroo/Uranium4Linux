#![allow(non_snake_case)]
use crate::checker::olog;
use code_functions::{fix_path, get_bool_element, get_parse_element, update, N_THREADS};
use downloaders::{
    curse_downloader::curse_modpack_downloader, minecraft_downloader,
    rinth_downloader::download_rinth_pack,
};
use modpack_maker::maker::make_modpack;
use searcher::rinth::SearchType;
use std::env;
use variables::constants::*;
use zip::result::ZipError;

mod checker;
mod code_functions;
mod downloaders;
mod easy_input;
mod hashes;
mod modpack_maker;
mod searcher;
mod variables;
mod zipper;

fn request_arg_parser(args: &[String]) -> Option<SearchType> {
    match args.iter().position(|f| f == REQUEST || f == LONG_REQUEST) {
        Some(index) => match args[index + 1].as_str() {
            QUERY => Some(SearchType::QUERY(args[index + 2].clone())),
            FOR => Some(SearchType::FOR(
                args[index + 2]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 2])),
                args[index + 3]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
            )),
            VERSION => Some(SearchType::VERSION(args[index + 1].clone())),
            VERSIONS => Some(SearchType::VERSIONS(args[index + 1].clone())),
            MOD => Some(SearchType::MOD(args[index + 1].clone())),
            PROJECT => Some(SearchType::PROJECT(args[index + 1].clone())),
            RESOURCEPACKS => Some(SearchType::RESOURCEPACKS(
                args[index + 2]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
                args[index + 3]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
            )),

            MODPACKS => Some(SearchType::MODPACKS(
                args[index + 2]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
                args[index + 3]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
            )),

            _ => panic!("Invalid request type !"),
        },
        None => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), ZipError> {
    #[cfg(debug_assertions)]
    olog("Debug enable");

    let args: Vec<String> = env::args().collect();

    {
        let mut aux = NTHREADS.write().unwrap();
        *aux = get_parse_element(&args, THREADS_FLAG).unwrap_or(DEFAULT_NTHREADS);
    }

    let file_path: String = get_parse_element(&args, FILE_FLAG).unwrap_or_default();
    let mut destination_path: String = get_parse_element(&args, ROOT_FLAG).unwrap_or_default();
    let instance: String = get_parse_element(&args, LONG_INSTACIATE).unwrap_or_default();

    let curse_pack = get_bool_element(&args, CURSE_FLAG);
    let rinth_pack = get_bool_element(&args, RINTH_FLAG);

    destination_path = fix_path(&destination_path);

    // TODO! Replace manual argument parse with CLAP. Or not...
    match args[1].as_str() {
        DOWNLOAD | LONG_DOWNLOAD => {
            if rinth_pack {
                download_rinth_pack(&file_path, &destination_path).await;
            } else if curse_pack {
                curse_modpack_downloader(&file_path, &destination_path).await;
            } else {
                println!("No repo found!");
            }
        }
        LONG_UPDATE => update(&file_path).await,
        MAKE | LONG_MAKE => make_modpack(&file_path, N_THREADS()).await,
        LONG_REQUEST => {
            searcher::rinth::search(request_arg_parser(&args).expect("Wrong request type")).await;
        }
        INSTACIATE | LONG_INSTACIATE => {
            minecraft_downloader::donwload_minecraft(&instance, destination_path.into())
                .await
                .unwrap()
        }
        LIST_INSTANCES => {
            minecraft_downloader::print_instances().await.unwrap();
        }
        HELP | LONG_HELP => println!("{}", HELP_MSG),
        _ => println!("Invalid arguments"),
    }
    Ok(())
}
