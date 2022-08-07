use reqwest::{header::HeaderMap, Response};
use super::load_headers;
use tokio::task::{JoinHandle, spawn};
use crate::mod_searcher::Method;

pub struct CurseRequester{ 
    cliente: reqwest::Client, 
    headers: HeaderMap,
} 

impl CurseRequester { 
    pub fn new() -> CurseRequester { 
        let mut req = CurseRequester{ 
            cliente: reqwest::Client::new(), 
            headers: HeaderMap::new(),
        };

        req.headers.insert("x-api-key", "$2a$10$6mO.gbGdm7elhecL3XMcxOby5RrftY2ufGTZxg3gocM1kDlF1UCuK".parse().unwrap());
        req.headers.insert("Content-Type", "application/json".parse().unwrap());
        req.headers.insert("Accept", "application/json".parse().unwrap());
        
        req 
    } 
    
    pub fn get(&self, url: &str, method: Method, body: &str) 
-> JoinHandle<Result<reqwest::Response, reqwest::Error>>{

        let url  = url.to_owned();
        let body = body.to_owned();
    
        let a_func; 
        match method {
            Method::GET  => a_func = self.cliente.get(&url).headers(self.headers.clone()).send(), 
            Method::POST => a_func = self.cliente.post(&url).headers(self.headers.clone()).body(body).send()
        }

        let task = spawn(a_func);
        task
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

     pub async fn get_curse(&self, url: String, method: &str, body: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let resp: Response;
        let body = body.to_owned();
        let mut headers = HeaderMap::new();
        load_headers::load_headers("CURSE", &mut headers);
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        match method {
            "post" => {
                let a = self.cliente.post(url).headers(self.headers.clone().unwrap_or_default()).body(body);
                println!("{:?}", a);
                resp = a.send().await?;
            },
            "get" => {
                resp = self.cliente.get(url).headers(self.headers.clone().unwrap_or_default()).send().await?; 
            },
            _ => {
                resp =  self.cliente.get(url).headers(self.headers.clone().unwrap_or_default()).send().await?;  
            }
        }
       Ok(resp)    
     }
 
    

    pub async fn get(&self, url: String) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let resp: Response;
        match self.headers.clone() {
            Some(h) => {
                resp = self.cliente.get(url).headers(h).send().await?;
            }
            None => {
                resp = self.cliente.get(url).send().await?;
            }
        }
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
