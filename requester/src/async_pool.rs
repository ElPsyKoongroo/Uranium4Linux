use std::{collections::HashMap, time::Duration};
use std::collections::VecDeque;
use tokio::{task::JoinHandle, time};
use tokio::task::JoinSet;
const PRE_TIME: u64 = 100;

// This is not anymore an AsyncPool, just a task sorter xd
pub struct AsyncPool<T> {
    request_pool: VecDeque<JoinHandle<T>>,
    items: usize,
    ordered_requests: HashMap<usize, T>,
}

impl<T> AsyncPool<T> {
    pub fn new() -> AsyncPool<T> {
        AsyncPool {
            request_pool: VecDeque::new(),
            items: 0,
            ordered_requests: HashMap::new(),
        }
    }

    pub fn push_request(&mut self, request: JoinHandle<T>) {
        self.request_pool.push_back(request);
    }

    pub fn push_request_vec(&mut self, new_requests: Vec<JoinHandle<T>>) {
        self.request_pool.append(&mut VecDeque::from(new_requests));
        if self.request_pool.len() > self.ordered_requests.capacity() {
            self.ordered_requests = HashMap::with_capacity(self.request_pool.len());
        }
    }

    pub fn clear(&mut self) {
        self.request_pool.clear();
        self.ordered_requests.clear();
    }

    pub async fn start(&mut self) -> Vec<T> where T: 'static + Send {
        self.items = self.request_pool.len();
        time::sleep(Duration::from_millis(PRE_TIME)).await;
        let mut join_set = JoinSet::new();

        for i in 0..self.items {
            let task = self.request_pool.pop_front().unwrap();
            join_set.spawn(async move {(task.await, i)});
        }

        while let Some(Ok((source,i ))) = join_set.join_next().await {
            self.ordered_requests.insert(i, source.unwrap());
        }
        (0..self.items).into_iter().map(|i| self.ordered_requests.remove(&i).unwrap()).collect()
    }
}
