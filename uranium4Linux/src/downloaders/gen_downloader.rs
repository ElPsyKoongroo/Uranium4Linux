use crate::code_functions::N_THREADS;
use futures::future::join_all;
use log::error;
use requester::{mod_searcher::Method, requester::request_maker::Req};
use reqwest::Response;
use std::{collections::VecDeque, error::Error, io::Write, path::PathBuf, sync::Arc};

/*
 *           +------------+
 *           | Downloader |
 *           +------------+
 *                 |                   path
 *                 V                   names
 *          +-----------------+        response_chunk       +--------------------+
 *          |   get_response  | --------------------------> | download_and_write |
 *          +-----------------+          *NEW TASK*         +--------------------+
 *              ^        |
 *              \       /
 *               \_____/
 *               ^^^^^^^
 *       for chunk in urls.chunk(N_THREADS)
 *
 * */

pub struct Downloader<T: Req + Clone + Send> {
    pub urls: Arc<Vec<String>>,
    pub names: Vec<PathBuf>,
    pub path: Arc<PathBuf>,
    pub requester: T,
}

impl<T: Req + Clone + std::marker::Send + std::marker::Sync + 'static> Downloader<T> {
    pub async fn start(mut self) {
        self.get_responses().await.unwrap();
    }

    async fn get_responses(&mut self) -> Result<(), Box<dyn Error>> {
        let mut tasks = Vec::new();
        let chunk_size = N_THREADS();

        for url_chunk in self.urls.chunks(chunk_size) {
            let path_c = self.path.clone();
            let names: Vec<PathBuf> = self.names.drain(0..url_chunk.len()).collect();

            let mut requests_vec = Vec::new();
            for url in url_chunk {
                let rq = self.requester.clone();
                let u = url.clone();
                requests_vec.push(async move { rq.get(&u, Method::GET, "").await.unwrap() });
            }

            let responses = join_all(requests_vec).await.into_iter().flatten().collect();

            let task = tokio::task::spawn(async {
                Downloader::<T>::download_and_write(path_c, responses, names)
                    .await
                    .map_err(|e| eprintln!("{}", e))
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
        names: Vec<PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        assert_eq!(responses.len(), names.len());
        for (i, response) in responses.into_iter().enumerate() {
            let bytes = response.bytes().await?;
            let file_path = path.join(&names[i]);
            let mut file = std::io::BufWriter::new(
                std::fs::File::create(&file_path)
                    //.await
                    .unwrap_or_else(|_| panic!("File {:?} cant be create", file_path)),
            );
            match file.write_all(&bytes) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "Can not write in {:?}: {}",
                        file_path.file_name().unwrap_or_default(),
                        e
                    );
                    return Err(e.into());
                }
            };
        }

        Ok(())
    }
}

pub struct Downloader2<T: Req + Clone + Send> {
    pub urls: Arc<Vec<String>>,
    pub names: Vec<PathBuf>,
    pub path: Arc<PathBuf>,
    pub requester: T,
    tasks: VecDeque<tokio::task::JoinHandle<()>>,
}

impl<T: Req + Clone + Send> Downloader2<T> {
    pub fn new(
        urls: Arc<Vec<String>>,
        names: Vec<PathBuf>,
        path: Arc<PathBuf>,
        requester: T,
    ) -> Downloader2<T> {
        Downloader2 {
            urls,
            names,
            path,
            requester,
            tasks: VecDeque::new(),
        }
    }
}

impl<T: Req + Clone + std::marker::Send + std::marker::Sync + 'static> Downloader2<T> {
    pub async fn start(&mut self) {
        //self.get_responses().await.unwrap();
    }

    #[allow(unused)]
    async fn get_responses(&mut self) -> Result<(), Box<dyn Error>> {
        let chunk_size = N_THREADS();

        for url_chunk in self.urls.chunks(chunk_size) {
            let path_c = self.path.clone();
            let names: Vec<PathBuf> = self.names.drain(0..url_chunk.len()).collect();

            let mut requests_vec = Vec::new();
            for url in url_chunk {
                let rq = self.requester.clone();
                let u = url.clone();
                requests_vec.push(async move { rq.get(&u, Method::GET, "").await.unwrap() });
            }

            let responses = join_all(requests_vec).await.into_iter().flatten().collect();

            let task = tokio::task::spawn(async {
                Downloader::<T>::download_and_write(path_c, responses, names)
                    .await
                    .map_err(|e| eprintln!("{}", e))
                    .unwrap();
            });

            self.tasks.push_back(task);
        }

        Ok(())
    }

    async fn make_requests(&mut self) -> Option<()> {
        let mut chunk_size = N_THREADS();

        if chunk_size > self.names.len() {
            chunk_size = self.names.len();
        }

        let path = self.path.clone();
        let start = self.urls.len() - self.names.len();
        let names = self.names.drain(0..chunk_size).collect();
        let urls = &self.urls[start..start + chunk_size];

        let mut requests_vec = Vec::new();
        for url in urls {
            let rq = self.requester.clone();
            let u = url.clone();
            requests_vec.push(async move { rq.get(&u, Method::GET, "").await.unwrap() });
        }

        let responses = join_all(requests_vec).await.into_iter().flatten().collect();

        let task = tokio::spawn(async {
            Downloader::<T>::download_and_write(path, responses, names)
                .await
                .map_err(|e| eprintln!("{}", e))
                .unwrap();
        });

        self.tasks.push_back(task);
        Some(())
    }

    pub async fn progress(&mut self) -> Option<usize> {
        if !self.names.is_empty() {
            self.make_requests().await.unwrap();
            return Some(self.names.len());
        } else if !self.tasks.is_empty() {
            for i in 0..self.tasks.len() {
                if self.tasks.get(i).unwrap().is_finished() {
                    let task = self.tasks.remove(i).unwrap();
                    task.await.unwrap();
                    self.tasks.len();
                }
            }
            let _ = self.tasks.pop_front().unwrap().await;
            return Some(self.tasks.len());
        }
        None
    }
}
