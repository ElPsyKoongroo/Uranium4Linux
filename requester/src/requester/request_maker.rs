use bytes::Bytes;
use reqwest::header::HeaderMap;
use tokio::task;
use tokio::task::{spawn, JoinHandle};

use crate::mod_searcher::Method;

use super::load_headers;

pub trait Req {
    fn get(&self, url: &str, method: Method, body: &str) -> task::JoinHandle<Result<reqwest::Response, reqwest::Error>>; 
}

pub struct RinthRequester {
    cliente: reqwest::Client,
    headers: HeaderMap,
}

impl RinthRequester {
    pub fn new() -> RinthRequester {
        let mut req = RinthRequester {
            cliente: reqwest::Client::new(),
            headers: HeaderMap::new(),
        };

        req.headers.insert(
            "x-api-key",
            "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".parse().unwrap(),
        );
        req.headers
            .insert("Content-Type", "application/json".parse().unwrap());
        req.headers
            .insert("Accept", "application/json".parse().unwrap());

        req
    }
    pub fn search_by_url(
        &self,
        url: &str,
    ) -> task::JoinHandle<Result<reqwest::Response, reqwest::Error>> {
        let url = url.to_owned();
        tokio::task::spawn(self.cliente.get(url).headers(self.headers.clone()).send())
    }
}

#[derive(Clone)]
pub struct CurseRequester {
    cliente: reqwest::Client,
    headers: HeaderMap,
}

unsafe impl Send for CurseRequester{}

impl CurseRequester {
    pub fn new() -> CurseRequester {
        let mut req = CurseRequester {
            cliente: reqwest::Client::new(),
            headers: HeaderMap::new(),
        };

        req.headers.insert(
            "x-api-key",
            "$2a$10$6mO.gbGdm7elhecL3XMcxOby5RrftY2ufGTZxg3gocM1kDlF1UCuK"
                .parse()
                .unwrap(),
        );
        req.headers
            .insert("Content-Type", "application/json".parse().unwrap());
        req.headers
            .insert("Accept", "application/json".parse().unwrap());

        req
    }  
}


impl Req for CurseRequester {

    fn get(
        &self,
        url: &str,
        method: Method,
        body: &str,
    ) -> JoinHandle<Result<reqwest::Response, reqwest::Error>> {
        let url = url.to_owned();
        let body = body.to_owned();

        let a_func = match method {
            Method::GET => self.cliente.get(&url).headers(self.headers.clone()).send(),
            Method::POST => self
                .cliente
                .post(&url)
                .headers(self.headers.clone())
                .body(body)
                .send(),
        };

        spawn(a_func)
    }
    
}

pub struct Requester {
    cliente: reqwest::Client,
    headers: Option<HeaderMap>,
}

impl Requester {
    pub fn new() -> Requester {
        let mut headers = HeaderMap::new();
        load_headers::load_headers("RINTH", &mut headers);
        Requester {
            cliente: reqwest::Client::new(),
            headers: Some(headers),
        }
    }

    pub async fn get_curse(
        &self,
        url: String,
        method: &str,
        body: &str,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let body = body.to_owned();
        let mut headers = HeaderMap::new();
        load_headers::load_headers("CURSE", &mut headers);
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        let resp = match method {
            "post" => {
                let a = self
                    .cliente
                    .post(url)
                    .headers(self.headers.clone().unwrap_or_default())
                    .body(body);
                println!("{:?}", a);
                a.send().await?
            }
            "get" => {
                self.cliente
                    .get(url)
                    .headers(self.headers.clone().unwrap_or_default())
                    .send()
                    .await?
            }
            _ => {
                self.cliente
                    .get(url)
                    .headers(self.headers.clone().unwrap_or_default())
                    .send()
                    .await?
            }
        };
        Ok(resp)
    }

    pub async fn get(&self, url: String) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let resp = match self.headers.clone() {
            Some(h) => self.cliente.get(url).headers(h).send().await?,
            None => self.cliente.get(url).send().await?,
        };
        Ok(resp)
    }

    #[allow(dead_code)]
    pub async fn get_with_headers(
        &self,
        url: String,
        headers: HeaderMap,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let resp = self.cliente.get(url).headers(headers).send().await?;
        Ok(resp)
    }

    pub fn set_headers(&mut self, headers: HeaderMap) {
        self.headers = Some(headers);
    }
}
