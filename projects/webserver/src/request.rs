use std::collections::HashMap;
use serde_json::{Value, from_str, json};

pub struct Request {
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Value,
    pub route_params: HashMap<String, String>,
    pub method: String,
    pub path: String
}

impl Request {
    fn new(method: String, path: String) -> Self {

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

    pub fn from(request: &str) -> Result<Self, String> {

        let parts: Vec<&str> = request.split("\n\r\n").collect();
        let mut request_headers: Vec<&str> = parts[0].split("\n").collect();
        request_headers.remove(0); // remove request line
        let mut headers: HashMap<String, String> = HashMap::new();

        for header in &request_headers {
            let header: Vec<&str> = header.split(": ").collect();
            if header.len() == 2 {
                headers.insert(String::from(header[0]), String::from(header[1]));
            }
        }

        let parsed: Vec<&str> = request.split_whitespace().collect();
        if parsed.len() > 1 {
            let method = String::from(parsed[0]);
            let path = String::from(parsed[1]);

            let mut body = json!({});
            if parts.len() > 1 {
                let mut message_body = parts[1];

                if let Some(body_end) = message_body.rfind("}") {
                    message_body = &message_body[0..(body_end+1)];
                }

                if let Ok(http_body) = from_str(message_body) {
                    body = http_body;
                } else {
                    body = json!({});
                }
            }

            let req = Request {
                method,
                query_params: HashMap::new(),
                path,
                headers,
                body,
                route_params: HashMap::new(),
            };

            return Result::Ok(req);
        } else {
            return Result::Err(format!("Invalid request format: {}", request));
        }
    }
}