use super::gen_downloader::*;
use crate::{
    checker::dlog,
    code_functions::N_THREADS,
    error::ModpackError,
    variables::constants::{RINTH_JSON, TEMP_DIR},
    zipper::pack_unzipper::unzip_temp_pack,
};
use mine_data_strutcs::rinth::rinth_packs::{load_rinth_pack, RinthMdFiles, RinthModpack};
use requester::requester::request_maker::RinthRequester;
use std::path::{Path, PathBuf};

/// RinthDownloader struct is responsable for downloading
/// the fiven modpack
pub struct RinthDownloader {
    gen_downloader: Downloader2<RinthRequester>,
    modpack: RinthModpack,
}

impl RinthDownloader {
    pub fn new<I: AsRef<Path>>(
        modpack_path: I,
        destination: I,
    ) -> Result<Self, ModpackError> {
        let modpack = RinthDownloader::load_pack(modpack_path)?;
        let (links, names) = RinthDownloader::get_data(&modpack);

        let destination = destination.as_ref().to_owned();

        if !destination.join("mods").exists() {
            std::fs::create_dir(destination.join("mods"))
                .map_err(|_| ModpackError::CantCreateDir)?;
        }

        if !destination.join("config").exists() {
            std::fs::create_dir(destination.join("config"))
                .map_err(|_| ModpackError::CantCreateDir)?;
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

    /// Returns the number of **CHUNKS** to download.
    ///
    /// So, if `N_THREADS` is set to 2 and there are 32 mods it
    /// will return 16;
    ///
    /// 32/2 = 16
    pub fn len(&self) -> usize {
        self.gen_downloader.urls.len() / N_THREADS()
    }

    /// Simply returns the modpack name.
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

    /// Starts the downloading.
    pub async fn start(&mut self) {
        self.gen_downloader.start().await;
    }

    /// Make progress.
    ///
    /// If the download still in progress return
    /// the number of chunks remaining.
    ///
    /// Else return None.
    pub async fn chunk(&mut self) -> Option<usize> {
        self.gen_downloader.progress().await
    }
}
