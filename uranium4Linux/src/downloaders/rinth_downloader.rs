use super::gen_downloader::Downloader;
use crate::{
    checker::dlog,
    downloaders::functions::overrides,
    variables::constants::{RINTH_JSON, TEMP_DIR},
    zipper::pack_unzipper::unzip_temp_pack,
};
use mine_data_strutcs::rinth::rinth_packs::{load_rinth_pack, RinthMdFiles};
use requester::requester::request_maker::RinthRequester;
use std::{path::PathBuf, sync::Arc};

pub async fn download_rinth_pack(path: &str, destination_path: &str) {
    unzip_temp_pack(path);
    let rinth_pack = load_rinth_pack(&(TEMP_DIR.to_owned() + RINTH_JSON));
    dlog("Pack loaded");

    let file_links: Vec<String> = rinth_pack
        .get_files()
        .iter()
        .map(RinthMdFiles::get_download_link)
        .collect();
    dlog(format!("Downloading {} files", file_links.len()));

    let file_names: Vec<PathBuf> = rinth_pack
        .get_files()
        .iter()
        .map(RinthMdFiles::get_name)
        .collect();

    file_names
        .iter()
        .for_each(|f| dlog(format!("{}", f.display())));

    let requester = RinthRequester::new();

    let downloader = Downloader {
        names: file_names,
        urls: Arc::new(file_links),
        path: Arc::new(PathBuf::from(destination_path)),
        requester,
    };

    downloader.start().await;
    overrides(&destination_path.into(), "overrides");
}
