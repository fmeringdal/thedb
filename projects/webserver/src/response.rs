use serde_json::{Value, to_string};

pub struct Response {
    status: u32,
    status_message: String,
    json: String
} 

impl Response {
    pub fn new() -> Self {
        Response {
            status: 404,
            status_message: String::from("Not Found"),
            json: String::from("")
        }
    }

    pub fn status(&mut self, status: u32){
        let (status_message, status) = match status {
            200 => ("Ok", status),
            201 => ("Created", status),
            204 => ("No Content", status),
            304 => ("Not Modified", status),
            400 => ("Bad Request", status),
            401 => ("Unauthorized", status),
            403 => ("Forbidden", status),
            404 => ("Not Found", status),
            409 => ("Conflict", status),
            500 => ("Interval Server Error", status),
            _ => ("Not Found", 404),
        };
        
        self.status_message = String::from(status_message);
        self.status = status;
    }

    pub fn get_status(&self) -> u32 {
        return self.status;
    }

    pub fn get_status_message(&self) -> &String {
        return &self.status_message;
    }

    pub fn send(&mut self, value: &str){
        self.status(200);
        self.json = String::from(value);
    }

    pub fn json(&mut self, value: &Value){
        self.status(200);
        if let Ok(response_message) = to_string(value) {
            self.json = response_message;
        }
    }

    pub fn get_json(&self) -> &String {
        return &self.json;
    }

    pub fn get_http_response(&self) -> String {
        let status = self.get_status();
        let status_message = self.get_status_message();
        let status_line = &format!("HTTP/1.1 {}\r\n", status);

        let message_body = self.get_json();
        let content_type = &format!("content-length: {}\r\n", message_body.len());
        let content_length = &format!("content-type: {}\r\n", "application/json; charset=utf-8");
        let connection = &format!("connection: {}\r\n", "keep-alive");
        let access_control = &format!("access-control-allow-origin: {}\r\n\r\n", "*");
        
        let response = format!("{}{}{}{}{}{}", 
            status_line,
            content_length,
            content_type,
            connection,
            access_control,
            message_body
        );

        return response;
    }
}