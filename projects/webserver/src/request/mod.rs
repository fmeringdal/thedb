use std::collections::HashMap;

pub struct Request {
    headers: u32,
    query_params: u32,
    body_params: u32,
    route_params: HashMap<String, String>,
    pub method: String,
    pub path: String
}

impl Request {
    pub fn new(method: String, path: String) -> Self {
        Request {
            headers: 0,
            query_params: 0,
            body_params: 0,
            route_params: HashMap::new(),
            method,
            path
        }
    }

    pub fn insert_route_param(&mut self, name: String, value: String){
        self.route_params.insert(name, value);
    }

    pub fn route_params(&self) -> &HashMap<String, String> {
        
        return &self.route_params;
    }
}