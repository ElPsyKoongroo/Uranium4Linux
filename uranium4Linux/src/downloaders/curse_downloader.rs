use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use crate::checker::{dlog, olog};
use crate::code_functions::N_THREADS;
use crate::variables::constants::TEMP_DIR;
use crate::zipper::pack_unzipper::unzip_temp_pack;
use bytes::Bytes;
use mine_data_strutcs::url_maker::maker::Curse;
use mine_data_strutcs::{curse::curse_modpacks::*, curse::curse_mods::*};
use requester::requester::request_maker::Req;
use requester::{
    async_pool::AsyncPool, mod_searcher::Method, requester::request_maker::CurseRequester,
};
use reqwest::Response;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinSet;

use super::functions::{get_writters, overrides, Writter};

struct Downloader<T: Req + Clone + Send> {
    urls: Arc<Vec<String>>,
    names: Vec<String>,
    requests: Arc<Vec<Response>>,
    path: Arc<PathBuf>,
    requester: T,
}

impl<T: Req + Clone + std::marker::Send + 'static> Downloader<T> {
    async fn start(mut self) {
        self.get_responses().await.unwrap();
    }

    async fn get_responses(&mut self) -> Result<(), Box<dyn Error>> {
        let mut i = 0;
        let mut tasks = Vec::new();
        let chunk_size = N_THREADS();
        for url_chunk in self.urls.chunks(chunk_size) {
            println!("{}", i);
            i += 1;
            let mut responses = Vec::with_capacity(self.urls.len());
            let path_c = self.path.clone();
            let names: Vec<String> = self.names.drain(0..url_chunk.len()).collect();

            let mut join_set = JoinSet::new();
            for url in url_chunk {
                let rq = self.requester.clone();
                let u = url.clone();
                join_set.spawn(async move {rq.get(&u, Method::GET, "")});
            }

            while let Some(Ok(r)) = join_set.join_next().await {
                responses.push(r.await??)
            }

            let task = tokio::task::spawn(async {
                Downloader::<T>::download_and_write(path_c, responses, names)
                    .await
                    .unwrap();
            });

            tasks.push(task);
        }
        for task in tasks {
            task.await?;
        }

        Ok(())
    }

    async fn download_and_write(
        path: Arc<PathBuf>,
        responses: Vec<Response>,
        names: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        assert_eq!(responses.len(), names.len());
        for (i, response) in responses.into_iter().enumerate() {
            let bytes = response.bytes().await.unwrap_or_default();
            let file_path = path.join(&names[i]);
            //olog(format!("{:?}", file_path));
            let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(file_path).await?);
            file.write_all(&bytes).await?;
        }

        Ok(())
    }
}

struct CurseDownloader {
    urls: Arc<Vec<String>>,
    content: Vec<Bytes>,
    names: Arc<Vec<String>>,
    path: PathBuf,
    requester: CurseRequester,
}

impl CurseDownloader {
    pub fn new(
        urls: Vec<String>,
        names: Vec<String>,
        path: PathBuf,
        requester: CurseRequester,
    ) -> Self {
        CurseDownloader {
            urls: Arc::new(urls),
            content: Vec::new(),
            names: Arc::new(names),
            path,
            requester,
        }
    }
    pub async fn download(self) {
        let threads = N_THREADS();
        let elements_per_chunk = threads;
        let mut i = 0;
        #[cfg(feature = "console_output")]
        let mut percent = 0.0;

        let mut ranges: Vec<std::ops::Range<usize>> = Vec::new();

        while i < self.urls.len() {
            let range = if i + elements_per_chunk < self.urls.len() {
                i..i + elements_per_chunk
            } else {
                i..self.urls.len()
            };

            ranges.push(range);
            i += elements_per_chunk;
        }

        let mut tasks = Vec::new();
        for range_chunk in ranges {
            let url_c = self.urls.clone();
            let name_c = self.names.clone();
            //let rcc: Vec<std::ops::Range<usize>> = range_chunk.iter().cloned().collect();
            let mut a = Vec::new();
            a.push(range_chunk);
            let req = self.requester.clone();
            let p = self.path.clone();
            tasks.push(tokio::task::spawn(async {
                CurseDownloader::process_chunk(url_c, name_c, a, req, p).await
            }));
        }

        for task in tasks {
            let _ = task.await;
        }
    }

    async fn process_chunk(
        urls: Arc<Vec<String>>,
        names: Arc<Vec<String>>,
        range_chunk: Vec<std::ops::Range<usize>>,
        requester: CurseRequester,
        path: PathBuf,
    ) {
        let elements_per_chunk = N_THREADS();
        let mut join_set = JoinSet::new();
        let mut writters = Vec::new();
        for range in range_chunk {
            let mut responses = Vec::with_capacity(elements_per_chunk);
            for download_url in urls[range.clone()].iter() {
                let rc = requester.clone();
                let url = download_url.clone();
                join_set.spawn(async move {
                    rc.get(&url, Method::GET, "")
                        .await
                        .unwrap()
                        .unwrap()
                        .bytes()
                        .await
                        .unwrap_or_default()
                });
            }

            while let Some(Ok(e)) = join_set.join_next().await {
                responses.push(e)
            }
            println!("{:?}", range);
            let writter = Writter::new(responses, path.clone(), &names[range]);
            writters.push(writter.write());
        }

        for writter in writters {
            println!("Writter finished!");
            writter.await;
        }
    }
}

pub async fn curse_modpack_downloader(path: &str, destination_path: &str) {
    unzip_temp_pack(path);

    let curse_pack =
        load_curse_pack(&(TEMP_DIR.to_owned() + "manifest.json")).expect("Couldnt load the pack");

    let files_ids: Vec<String> = curse_pack
        .get_files()
        .iter()
        .map(|f| Curse::file(&f.get_projectID().to_string(), &f.get_fileID().to_string()))
        .collect();

    let curse_req = CurseRequester::new();

    // Get the info of each mod to get the url and download it
    let responses: Vec<Response> = get_mod_responses(&curse_req, files_ids).await;
    let mut names = Vec::new();

    let mods_path = destination_path.to_owned() + "mods/";

    let start = std::time::Instant::now();
    let download_urls = get_download_urls(&curse_req, responses, &mut names).await;
    let downloader = Downloader {
        names,
        urls: Arc::new(download_urls),
        path: Arc::new(PathBuf::from(mods_path)),
        requests: Arc::new(Vec::new()),
        requester: curse_req,
    };
    downloader.start().await;
    /*
    let cd = CurseDownloader::new(download_urls, names, mods_path.into(), curse_req);
    cd.download().await;
    */
    let end = std::time::Instant::now();

    olog(format!(
        "Download time: {}",
        end.duration_since(start).as_secs_f64()
    ));

    /*
    let responses = download_mods(&curse_req, download_urls, &names, &mods_path).await;

    let writter = Writter::new(responses, mods_path.into(), names);
    writter.write().await;
    */
    /*
    let writters = get_writters(responses, &names, &mods_path).await;
    let mut pool = AsyncPool::new();
    pool.push_request_vec(writters);
    pool.start().await;
    */
    overrides(&destination_path.into(), "overrides");
}

async fn get_mod_responses(curse_req: &CurseRequester, files_ids: Vec<String>) -> Vec<Response> {
    let mut responses: Vec<Response> = Vec::with_capacity(files_ids.len());

    // Split the files ids into chunks so Uranium dont spawn
    // 5784923543 threads

    for chunk in files_ids.chunks(N_THREADS()) {
        let mut pool = AsyncPool::new();
        let mut requests = Vec::new();
        for url in chunk {
            let tarea = curse_req.get(url, Method::GET, "");
            requests.push(tarea);
        }
        pool.push_request_vec(requests);
        // pool.start().await;

        // Wait for the current pool to end and append the results
        // to the results vector
        responses.append(
            &mut pool
                .start()
                .await
                .into_iter()
                .filter_map(|f| match f {
                    Ok(val) => Some(val),
                    Err(e) => {
                        println!("{:?}", e);
                        None
                    }
                })
                .collect(),
        );
        // pool.clear();
    }

    responses
}

#[allow(unused)]
async fn get_download_urls(
    curse_req: &CurseRequester,
    responses: Vec<Response>,
    names: &mut Vec<String>,
) -> Vec<String> {
    // In order to get rid of reallocations pre allocate the vector with
    // responses capacity.
    // The vector rarelly will get full beacause of empty links.
    let mut download_urls = Vec::with_capacity(responses.len());

    for response in responses {
        // Parse the response into a CurseResponse<CurseFile>
        let curse_file = response.json::<CurseResponse<CurseFile>>().await;
        if let Ok(file) = curse_file {
            let download_url = file.data.get_downloadUrl();

            // In case the download link its empty, because CurseApi could give
            // a right response but with empty download link... -.-
            if download_url.is_empty() {
                println!("There is no download link for {}", file.data.get_fileName());
            } else {
                names.push(file.data.get_fileName());
                download_urls.push(download_url);
            }
        }
    }
    download_urls
}

async fn download_mods(
    curse_req: &CurseRequester,
    download_urls: Vec<String>,
    names: &[String],
    _mods_path: &str,
) -> Vec<Bytes> {
    let _names_chunks = names.chunks(N_THREADS()).collect::<Vec<&[String]>>();
    let mut responses = Vec::with_capacity(download_urls.len());

    #[cfg(feature = "console_output")]
    let mut percent = 0.0;

    let mut join_set = JoinSet::new();
    for chunk in download_urls.chunks(N_THREADS()) {
        // Add the tasks for this chunk
        for download_url in chunk {
            let tarea = curse_req.get(download_url, Method::GET, "");
            join_set.spawn(tarea);
            // tareas.push(tarea);
        }
        //pool.push_request_vec(tareas);

        // Collect the responses and then push them into responses vector
        //let mut chunk_res: Vec<Response> = pool.start().await.into_iter().flatten().collect();

        while let Some(Ok(Ok(e))) = join_set.join_next().await {
            let b = match e {
                Ok(r) => r.bytes().await.unwrap(),
                Err(_) => bytes::Bytes::default(),
            };
            responses.push(b)
        }

        #[cfg(feature = "console_output")]
        {
            percent += chunk.len() as f32 / download_urls.len() as f32 * 100.0;
            println!("{:0.2} %", percent);
        }
    }
    responses
}
