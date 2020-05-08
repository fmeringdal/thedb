use std::collections::HashMap;
use serde_json::{Value, from_str};

pub struct Request {
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Value,
    pub route_params: HashMap<String, String>,
    pub method: String,
    pub path: String
}

impl Request {
    pub fn new(method: String, path: String) -> Self {

        let init_body = "{}";

        Request {
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: from_str(&init_body).unwrap(),
            route_params: HashMap::new(),
            method,
            path
        }
    }
}