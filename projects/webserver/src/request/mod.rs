use std::collections::HashMap;

pub struct Request {
    pub headers: HashMap<String, String>,
    query_params: u32,
    body_params: u32,
    pub route_params: HashMap<String, String>,
    pub method: String,
    pub path: String
}

impl Request {
    pub fn new(method: String, path: String) -> Self {
        Request {
            headers: HashMap::new(),
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