use reqwest::{header::HeaderMap, Response};
use super::load_headers;
use tokio::task::{JoinHandle, spawn};

pub enum CurseMethod{
    GET,
    POST 
} 
pub struct CurseRequester{ 
    cliente: reqwest::Client, 
    headers: HeaderMap 
} 

type Coso = JoinHandle<Result<reqwest::Response, reqwest::Error>>;

impl CurseRequester { 
    pub fn new() -> CurseRequester { 
        let mut headers = HeaderMap::new(); 
        load_headers::load_headers("CURSE", &mut headers); 
        CurseRequester{ 
            cliente: reqwest::Client::new(), 
            headers: headers 
        } 
    } 
    
    pub async fn get(&self, url: String, method: CurseMethod, body: &str) -> JoinHandle<Result<reqwest::Response, reqwest::Error>>
{
        let url  = url.to_owned();
        let body = body.to_owned();
        
        let mut new_header = HeaderMap::new();
        new_header.insert("x-api-key", "$2a$10$6mO.gbGdm7elhecL3XMcxOby5RrftY2ufGTZxg3gocM1kDlF1UCuK".parse().unwrap());

        let a_func; 
        match method {
            CurseMethod::GET  => a_func = self.cliente.get(&url).headers(new_header).send(), 
            CurseMethod::POST => a_func = self.cliente.post(&url).body(body).send()
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
        match method {
            "post" => {
                resp = self.cliente.post(url).headers(self.headers.clone().unwrap_or_default()).body(body).send().await?;
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
