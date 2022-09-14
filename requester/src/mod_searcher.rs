use crate::requester::request_maker::CurseRequester;
use mine_data_strutcs::url_maker::maker::ModRinth;
use tokio::task;
use tokio::task::JoinHandle;

pub enum Method {
    GET,
    POST,
}

pub struct RequestInfo {
    pub url: String,
    pub method: Method,
    pub body: String,
}

pub fn search_with_client(
    requester: &CurseRequester,
    request: RequestInfo,
) -> JoinHandle<Result<reqwest::Response, reqwest::Error>> {
    match request.method {
        Method::GET => requester.get(&request.url, Method::GET, ""),
        Method::POST => requester.get(&request.url, Method::POST, &request.body),
    }
}

pub fn search_mod_by_id(id: &str) -> task::JoinHandle<reqwest::Response> {
    let url = ModRinth::mod_versions_by_id(id);
    let a_func = async {
        let cliente = reqwest::Client::new();
        cliente.get(url).send().await.unwrap()
    };
    task::spawn(a_func)
}

pub fn search_version_by_id(id: &str) -> task::JoinHandle<reqwest::Response> {
    let url = ModRinth::mod_version_by_id(id);
    let a_func = async {
        let cliente = reqwest::Client::new();
        cliente.get(url).send().await.unwrap()
    };
    task::spawn(a_func)
}

pub fn search_by_url(url: &str) -> task::JoinHandle<reqwest::Response> {
    let url = url.to_owned();
    let a_func = async move {
        let cliente = reqwest::Client::new();
        cliente.get(&url).send().await.unwrap()
    };
    task::spawn(a_func)
}
