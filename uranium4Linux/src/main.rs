#![allow(non_snake_case)]
#![forbid(unsafe_code)]

#[cfg(debug_assertions)]
use uranium4Linux::checker::olog;

use code_functions::{fix_path, get_bool_element, get_parse_element, update};
use downloaders::{curse_downloader::curse_modpack_downloader, minecraft_downloader};
use std::env;
use uranium4Linux::{request_arg_parser, *};
use variables::constants::*;
use zip::result::ZipError;

#[tokio::main]
async fn main() -> Result<(), ZipError> {
    #[cfg(debug_assertions)]
    olog("Debug enable");

    let args: Vec<String> = env::args().collect();

    set_threads(get_parse_element(&args, THREADS_FLAG).unwrap_or(DEFAULT_NTHREADS));

    let file_path: String = get_parse_element(&args, FILE_FLAG).unwrap_or_default();
    let mut destination_path: String = get_parse_element(&args, ROOT_FLAG).unwrap_or_default();
    let instance: String = get_parse_element(&args, LONG_INSTACIATE).unwrap_or_default();

    let curse_pack = get_bool_element(&args, CURSE_FLAG);
    let rinth_pack = get_bool_element(&args, RINTH_FLAG);

    destination_path = fix_path(&destination_path);

    // TODO! Replace manual argument parse with CLAP. Or not...
    match args[1].as_str() {
        SHORT_DOWNLOAD | LONG_DOWNLOAD => {
            if rinth_pack {
                let _ = rinth_pack_download(file_path, destination_path).await;
            } else if curse_pack {
                let _ = curse_modpack_downloader(&file_path, &destination_path).await;
            } else {
                println!("No repo found!");
            }
        }
        SHORT_UPDATE | LONG_UPDATE => update(&file_path).await,
        SHORT_MAKE | LONG_MAKE => {make_modpack(&file_path).await;}
        SHORT_REQUEST | LONG_REQUEST => {
            searcher::rinth::search(request_arg_parser(&args).expect("Wrong request type")).await;
        }
        SHORT_INSTACIATE | LONG_INSTACIATE => minecraft_downloader::donwload_minecraft(
            &instance,
            destination_path
                .try_into()
                .expect("Impossible to parse destination path!"),
        )
        .await
        .expect("Couldnt download this minecraft version!"),
        LIST_INSTANCES => {
            minecraft_downloader::print_instances().await.unwrap();
        }
        SHORT_HELP | LONG_HELP => println!("{}", HELP_MSG),
        _ => println!("Invalid arguments\n\n {}", HELP_MSG),
    }
    Ok(())
}
