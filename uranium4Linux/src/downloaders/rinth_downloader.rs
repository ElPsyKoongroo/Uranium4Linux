use super::gen_downloader::*;
use crate::{
    checker::dlog,
    code_functions::N_THREADS,
    downloaders::functions::overrides,
    error::ModpackError,
    variables::constants::{RINTH_JSON, TEMP_DIR},
    zipper::pack_unzipper::unzip_temp_pack,
};
use mine_data_strutcs::rinth::rinth_packs::{load_rinth_pack, RinthMdFiles, RinthModpack};
use requester::requester::request_maker::RinthRequester;
use std::{
    path::{Path, PathBuf},
    sync::Arc 
};

pub struct RinthDownloader {
    gen_downloader: Downloader2<RinthRequester>,
    modpack: RinthModpack,
}

impl RinthDownloader {
    pub fn new<I: Into<PathBuf>>(
        modpack_path: &I,
        destination: &I,
    ) -> Result<Self, ModpackError> where I: AsRef<Path> {
        let modpack = RinthDownloader::load_pack(modpack_path)?;
        let (links, names) = RinthDownloader::get_data(&modpack);

        let destination = destination.as_ref().to_owned();

        if !destination.join("mods").exists() {
            std::fs::create_dir(destination.join("mods")).map_err(|_| ModpackError::CantCreateDir)?
        }

        if !destination.join("config").exists() {
            std::fs::create_dir(destination.join("config")).map_err(|_| ModpackError::CantCreateDir)?
        }

        Ok(RinthDownloader {
            gen_downloader: Downloader2::new(
                links.into(),
                names,
                destination.into(),
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
            .map(RinthMdFiles::get_name)
            .collect();

        file_names
            .iter()
            .for_each(|f| dlog(format!("{}", f.display())));

        (file_links, file_names)
    }

    fn load_pack<I: AsRef<Path>>(path: I) -> Result<RinthModpack, ModpackError> {
        unzip_temp_pack(path)?;
        let Some(rinth_pack) = load_rinth_pack(&(TEMP_DIR.to_owned() + RINTH_JSON)) else {
            panic!("Cant read the pack")};

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

pub async fn download_rinth_pack<I: AsRef<Path> + std::fmt::Debug>(
    path: I,
    destination_path: I,
) -> Result<(), ModpackError>
where
    PathBuf: From<I>,
{
    unzip_temp_pack(path)?;
    let Some(rinth_pack) = load_rinth_pack(&(TEMP_DIR.to_owned() + RINTH_JSON)) else {
         return Err(ModpackError::WrongModpackFormat)  };

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
        path: Arc::new(destination_path.as_ref().into()),
        requester,
    };

    downloader.start().await;
    overrides(&destination_path.into(), "overrides");
    Ok(())
}
