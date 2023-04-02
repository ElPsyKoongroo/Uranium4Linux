use crate::checker::{check, elog, log, olog};
use crate::variables::constants::TEMP_DIR;
use crate::N_THREADS;
use bytes::Bytes;
use reqwest::Response;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::RwLock;
use tokio::task::JoinHandle;

pub struct Writter<'a> {
    responses: Arc<Vec<Bytes>>,
    names: &'a[String],
    path: PathBuf,
}

impl<'a> Writter<'a> {
    pub fn new(responses: Vec<Bytes>, path: PathBuf, names: &'a[String]) -> Self {
        Writter {
            responses: Arc::new(responses),
            names,
            path,
        }
    }

    pub async fn write(self) {
        assert_eq!(self.responses.len(), self.names.len());
        let threads = N_THREADS();
        let elements_per_chunk = self.responses.len() / threads;

        for i in 0..threads {
            let range = (i * elements_per_chunk)..((i + 1) * elements_per_chunk);
            Writter::write_chunk(range, self.responses.clone(), self.path.clone(), self.names.clone()).await;
        }
    }

    async fn write_chunk(
        range: Range<usize>,
        responses: Arc<Vec<Bytes>>,
        path: PathBuf,
        names: &[String],
    ) {
        for i in range {
            let name: &str = &names[i];
            let response = &responses[i];

            let full_path = path.join(name);
            let content = response;

            match tokio::fs::write(&full_path, content).await {
                Ok(_) => {
                }
                Err(e) => {
                    elog(format!("Errow writting {:?}, {}", full_path, e));
                }
            }
        }
    }
}

pub async fn write_mod(path: &str, res: Response, name: &str) {
    let full_path = path.to_owned() + name;
    let content = res.bytes().await.unwrap();

    match tokio::fs::write(&full_path, content).await {
        Ok(_) => {}
        Err(e) => {
            elog(format!("Errow writting {}, {}", full_path, e));
        }
    }
}

pub async fn write_mod2(path: Arc<PathBuf>, res: Response, name: &str) {
    let full_path = (*path).join(name);
    let content = res.bytes().await.unwrap();

    match tokio::fs::write(&full_path, content).await {
        Ok(_) => {}
        Err(e) => {
            elog(format!("Errow writting {:?}, {}", full_path, e));
        }
    }
}

pub async fn get_writters<T: ToString>(
    responses: Vec<Response>,
    names: &[T],
    destination_path: &str,
) -> Vec<JoinHandle<()>> {
    let mut writters = Vec::with_capacity(responses.len());
    let path_arc = Arc::new(PathBuf::from(destination_path));
    for (i, response) in responses.into_iter().enumerate() {
        //let path_ref = destination_path.to_owned();
        let mod_name = names[i].to_string();
        let pc = path_arc.clone();
        let task = async move {
            write_mod2(pc, response, &mod_name).await;
        };
        writters.push(tokio::spawn(task));
    }
    writters
}

pub fn overrides(destination_path: &PathBuf, overrides_folder: &str) {
    // Copy all the content of overrides into the minecraft root folder
    let options = fs_extra::dir::CopyOptions::new();
    let mut file_options = fs_extra::file::CopyOptions::new();
    file_options.overwrite = true;
    let overrides_folder = TEMP_DIR.to_owned() + overrides_folder;

    let entries = match fs::read_dir(&overrides_folder) {
        Ok(e) => e,
        Err(error) => {
            // We dont care about this result, we are going to panic or just leave
            // this function in case there is an error so no need to manage it
            match error.kind() {
                std::io::ErrorKind::NotFound => elog("Error, no overrides folder"),
                std::io::ErrorKind::PermissionDenied => elog("Error permision deniend"),
                _ => elog("Error, cant write the file"),
            };
            // TODO! Fix this panic. Make the function return a result
            // and manage (or not) the error in parent functions
            panic!();
            // return;
        }
    };

    // Iter through the override directory and copy the content to
    // Minecraft Root (`destination_path`)
    for file in entries.flatten() {
        // There's no need to panick, ¿Is this a mess?
        // TODO! Check if file_type can actually panic here.
        if file.file_type().unwrap().is_dir() {
            check(
                fs_extra::dir::copy(file.path(), destination_path, &options),
                false,
                "functions: Failt to copy override file",
            )
            .ok();
        } else {
            let copy_status = std::fs::copy(&file.path(), destination_path.join(&file.file_name()));
            check(
                copy_status,
                false,
                &format!("Error coppying {:?}", file.path()),
            )
            .ok();
        }
    }
}
