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

    fn parse_headers(header_fields: &[&str]) -> HashMap<String, String>{        
        let mut headers: HashMap<String, String> = HashMap::new();
        for header in header_fields {
            let header: Vec<&str> = header.split(": ").collect();
            if header.len() == 2 {
                headers.insert(String::from(header[0]), String::from(header[1]));
            }
        }
        headers
    }

    fn parse_request_line(request_line: &str) -> Result<(&str, &str), &str> {
        let request_line: Vec<&str> = request_line.split_whitespace().collect();
        if request_line.len() > 1 {
            Result::Ok((request_line[0], request_line[1]))
        } else {
            println!("Invalid request line");
            Result::Err("Invalid request line.")
        }
    }

    fn parse_message_body_from(request_parts: &Vec<&str>) -> Value {
        let mut body = json!({});
        if request_parts.len() > 1 {
            let mut message_body = request_parts[1];
            if let Some(body_end) = message_body.rfind("}") {
                message_body = &message_body[0..(body_end+1)];
            }
            if let Ok(http_body) = from_str(message_body) {
                body = http_body;
            }
        }
        body
    }

    pub fn from(request: &str) -> Result<Self, String> {
        let request_parts: Vec<&str> = request.split("\n\r\n").collect();

        // BODY
        let body = Request::parse_message_body_from(&request_parts);
        
        // HEADERS
        let headers;
        let header: Vec<&str> = request_parts[0].split("\n").collect();
        if header.len() < 2 {
            headers = HashMap::new();
        } else {
            let header_fields = &header[1..];
            headers = Request::parse_headers(&header_fields);
        }

        // DATA FROM REQUEST LINE
        let path;
        let method;
        let request_line = header[0];
        if let Ok((req_method, req_path)) = Request::parse_request_line(request_line) {
            path = req_path;
            method = req_method;
        } else {
            return Result::Err(format!("Invalid request line: {}", request_line));
        }


        // CREATE REQUEST OBJECT FROM OBTAINED DATA
        let req = Request {
            method: String::from(method),
            query_params: HashMap::new(),
            path: String::from(path),
            headers,
            body,
            route_params: HashMap::new(),
        };

        Result::Ok(req)
    }
}