use std::{time::Duration, collections::HashMap};
use tokio::{task::JoinHandle, time};

const PRE_TIME: u64 = 100;

pub struct AsyncPool<T> {
    request_pool: Vec<JoinHandle<T>>,
    items: usize,
    not_done_request: Vec<usize>,
    ordered_requests: HashMap<usize, T>
}


impl<T> AsyncPool<T> {

    pub fn new() -> AsyncPool<T> {
        AsyncPool {
            request_pool: Vec::new(),
            items: 0,
            not_done_request: Vec::new(),
            ordered_requests: HashMap::new()
        }
    }

    pub fn push_request(&mut self, request: JoinHandle<T>) {
        self.request_pool.push(request);
    }

    pub fn push_request_vec(&mut self, mut _requests: Vec<JoinHandle<T>>){
        self.request_pool.append(&mut _requests);
    }

    pub async fn start(&mut self){
        self.items = self.request_pool.len();
        self.not_done_request = Vec::from_iter(0..self.items);
        time::sleep(Duration::from_millis(PRE_TIME)).await;
        while !self.not_done_request.is_empty() {
            self.request_loop().await;
        }
    }

    async fn request_loop(&mut self) {

        // let join_all = futures::future::join_all(&mut self.request_pool).await;
        // let a: Vec<&T> = join_all.iter().flatten().collect::<Vec<&T>>();
        
        // for (i, res) in (0..self.items).zip(a){
        //     self.ordered_requests.insert(i, res);
        // }

        for i in self.not_done_request.clone().into_iter() {
            let sleep = time::sleep(Duration::from_millis(20));
            tokio::pin!(sleep);

            tokio::select! {
                _ = &mut sleep =>  {
                    continue;
                }
                
                res = &mut self.request_pool[i] => {
                    self.ordered_requests.insert(i, res.unwrap());
                    self.not_done_request.retain(|&x| x != i);
                }
            }
        }
    }

    pub fn get_done_request(&mut self) -> Vec<T> {
        let mut done_requests = Vec::new();
        for i in 0..self.items{
            let value = self.ordered_requests.remove(&i).unwrap();
            done_requests.push(value);
        }
        done_requests
    }
}

