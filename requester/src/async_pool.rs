use std::time::Duration;

use reqwest::Response;
use tokio::{task::{JoinHandle}, time};

pub struct AsyncPool {
    request_pool: Vec<JoinHandle<Response>>,
    done_request: Vec<Response>,
    not_done_request: Vec<usize>,
}


impl AsyncPool {

    pub fn new() -> AsyncPool {
        AsyncPool {
            request_pool: Vec::new(),
            done_request: Vec::new(),
            not_done_request: Vec::new(),
        }
    }

    pub fn push_request(&mut self, request: JoinHandle<Response>) {
        self.request_pool.push(request);
    }

    pub async fn start(&mut self){
        self.done_request = Vec::with_capacity(self.request_pool.len());
        self.not_done_request = Vec::from_iter(0..self.request_pool.len());
        while !self.not_done_request.is_empty() {
            self.request_loop().await;
        }
    }

    async fn request_loop(&mut self) {
        for i in self.not_done_request.clone().iter() {
            let sleep = time::sleep(Duration::from_millis(50));
            tokio::pin!(sleep);

            tokio::select! {
                _ = &mut sleep =>  {
                    continue;
                }
                
                res = &mut self.request_pool[*i] => {
                    self.done_request.push(res.unwrap());
                    self.not_done_request.retain(|&x| x != *i);

                }
            }
        }
    }

    pub fn get_done_request(self) -> Vec<Response> {
        self.done_request
    }
}

