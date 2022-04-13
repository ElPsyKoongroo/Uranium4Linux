
pub struct async_pool {
    request_pool: Vec<JoinHandle<Response>>,
    done_request: Vec<Response>,
}


impl async_pool {

    pub fn make_requests(&mut self, urls: Vec<String>) {
        for url in urls {
            let request = async {
                let mut res = client.get(&url).send();
            };
            self.request_pool.push(request);
        }
    }

    pub fn push_request(&mut self, request: JoinHandle<Response>) {
        self.request_pool.push(request);
    }

    pub fn start(&mut self){
        done_request = Vec::new();
    }


    /*
        
    
    */
    
    // Vec<JoinHandle<Response>>


}