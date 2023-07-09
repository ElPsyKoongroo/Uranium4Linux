use super::gen_downloader::*;
use crate::{
    checker::dlog,
    downloaders::functions::overrides,
    error::ModpackError,
    variables::constants::{RINTH_JSON, TEMP_DIR},
    zipper::pack_unzipper::unzip_temp_pack, code_functions::N_THREADS,
};
use mine_data_strutcs::rinth::rinth_packs::{load_rinth_pack, RinthMdFiles, RinthModpack};
use requester::requester::request_maker::RinthRequester;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct RinthDownloader {
    gen_downloader: Downloader2<RinthRequester>,
    modpack: RinthModpack,
}

impl RinthDownloader {
    pub fn new<I: Into<PathBuf> + ToString>(
        modpack_path: I,
        destination: I,
    ) -> Result<Self, ModpackError> {
        let modpack = RinthDownloader::load_pack(modpack_path.to_string())?;
        let (links, names) = RinthDownloader::get_data(&modpack);

        Ok(RinthDownloader {
            gen_downloader: Downloader2::new(
                links.into(),
                names,
                Arc::new(destination.into()),
                RinthRequester::new(),
            ),
            modpack,
        })
    }

    pub fn len(&self) -> usize {
        self.gen_downloader.urls.len() / N_THREADS()
    }

    pub fn get_modpack_name(&self) -> String {
        self.modpack.get_name()
    }

    fn get_data(rinth_pack: &RinthModpack) -> (Vec<String>, Vec<PathBuf>) {
        let file_links: Vec<String> = rinth_pack
            .get_files()
            .iter()
            .map(RinthMdFiles::get_download_link)
            .collect();

        dlog(format!("Downloading {} files", file_links.len()));

        let file_names: Vec<PathBuf> = rinth_pack
            .get_files()
            .iter()
            .map(|f| f.get_name())
            .collect();

        file_names
            .iter()
            .for_each(|f| dlog(format!("{}", f.display())));

        (file_links, file_names)
    }

    fn load_pack<I: AsRef<Path> + std::fmt::Debug>(path: I) -> Result<RinthModpack, ModpackError> {
        unzip_temp_pack(path)?;
        let rinth_pack = match load_rinth_pack(&(TEMP_DIR.to_owned() + RINTH_JSON)) {
            Some(pack) => pack,
            None => panic!("Cant read the pack"),
        };

        dlog("Pack loaded");

        Ok(rinth_pack)
    }

    pub async fn start(&mut self) {
        self.gen_downloader.start().await;
    }

    pub async fn chunk(&mut self) -> Option<usize> {
        self.gen_downloader.progress().await
    }
}

pub async fn download_rinth_pack<I: AsRef<Path> + std::fmt::Debug>(path: I, destination_path: I) -> Result<(), ModpackError>
where
    PathBuf: From<I>,
{
    unzip_temp_pack(path)?;
    let rinth_pack = match load_rinth_pack(&(TEMP_DIR.to_owned() + RINTH_JSON)) {
        Some(pack) => pack,
        None => return Err(ModpackError::WrongModpackFormat),
    };

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
        .map(|f| f.get_name())
        .collect();

    file_names
        .iter()
        .for_each(|f| dlog(format!("{}", f.display())));

    let requester = RinthRequester::new();

    let downloader = Downloader {
        names: file_names,
        urls: Arc::new(file_links),
        path: Arc::new(destination_path.as_ref().into()),
        requester,
    };

    downloader.start().await;
    overrides(&destination_path.into(), "overrides");
    Ok(())
}
