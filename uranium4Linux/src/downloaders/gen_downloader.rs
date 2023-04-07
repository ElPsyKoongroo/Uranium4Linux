use requester::{requester::request_maker::Req, mod_searcher::Method};
use reqwest::Response;
use tokio::{task::JoinSet, io::AsyncWriteExt};
use std::{error::Error, path::PathBuf, sync::Arc};

use crate::code_functions::N_THREADS;

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
        let mut i = 0;
        let mut tasks = Vec::new();
        let chunk_size = N_THREADS();
        for url_chunk in self.urls.chunks(chunk_size) {
            println!("{}", i);
            i += 1;
            let mut responses = Vec::with_capacity(self.urls.len());
            let path_c = self.path.clone();
            let names: Vec<PathBuf> = self.names.drain(0..url_chunk.len()).collect();

            let mut join_set = JoinSet::new();
            for url in url_chunk {
                let rq = self.requester.clone();
                let u = url.clone();
                join_set.spawn(async move { rq.get(&u, Method::GET, "").await });
            }

            while let Some(Ok(r)) = join_set.join_next().await {
                responses.push(r??)
            }

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
            let bytes = response.bytes().await.unwrap_or_default();
            let file_path = path.join(&names[i]);
            let mut file = tokio::io::BufWriter::new(tokio::fs::File::create(&file_path).await?);
            match file.write_all(&bytes).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!(
                        "[ERROR] Can not write in {file_name}: {error}",
                        file_name = file_path.display(),
                        error = e
                    )
                    .into());
                }
            };
        }

        Ok(())
    }
}
