
use reqwest::header::{HeaderMap};


pub struct Requester{
    cliente: reqwest::Client
}

impl Requester{
    
    pub fn new() -> Requester{
        let reqter = Requester{cliente: reqwest::Client::new()};
        reqter
    }
    
    #[allow(dead_code)]
    pub async fn get(&self, url: String) -> Result<reqwest::Response, Box<dyn std::error::Error>>{
        let resp =self.cliente.get(url)
            .send()
            .await?;
        Ok(resp)
    }

    pub async fn get_with_headers(&self, url: String, headers: HeaderMap)-> Result<reqwest::Response, Box<dyn std::error::Error>>{
        let resp =self.cliente.get(url)
            .headers(headers)
            .send()
            .await?;
        Ok(resp)
    }
}