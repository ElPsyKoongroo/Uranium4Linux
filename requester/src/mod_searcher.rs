use tokio::task;
use mine_data_strutcs::url_maker::maker::ModRinth;


pub fn search_mod_by_id(id: &str) -> task::JoinHandle<reqwest::Response> {
    let url = ModRinth::mod_versions_by_id(id);
    let a_func = async {
            let cliente = reqwest::Client::new();
            cliente.get(url).send().await.unwrap()
    };
    let task = task::spawn(a_func);
    task
}