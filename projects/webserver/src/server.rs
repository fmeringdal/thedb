use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

use serde_json::{json, from_str};
use crate::thread_pool::ThreadPool;
use std::collections::HashMap;
use std::sync::Arc;

use crate::router::{Router, Controller, RouterService};

use crate::request::Request;
use crate::response::Response;

pub struct Server {
    // pool: ThreadPool,
    mount_router: Router
}

impl Server {
    pub fn new() -> Self {
        let mount_router = Router::new();

        return Server {
            // pool: pool,
            mount_router
        };
    }

    fn handle_connection(mut stream: TcpStream, server: Arc<Server>) {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();
    
        let request = String::from_utf8_lossy(&buffer);
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
        if parsed.len() < 2 {
            let response = format!("{}{}", 400, "Invalid request");

            stream.write(response.as_bytes()).unwrap();
        
            stream.flush().unwrap();
        } else {
            let method = String::from(parsed[0]);
            let path = String::from(parsed[1]);

            let mut body = json!({});
            if parts.len() > 1 {
                let mut message_body = parts[1];

                match message_body.rfind('}') {
                    Some(body_end) => {
                        message_body = &message_body[0..(body_end+1)];
                    },
                    None => println!("Error"),
                }

                body = match from_str(message_body) {
                    Ok(http_body) => http_body,
                    Err(_) => {
                        json!({})},
                } 
            }

            let mut req = Request::new(method, path);
            let mut res = Response::new();
            
            req.headers = headers;
            req.body = body;

            server.mount_router.handle_request(&mut req, &mut res, &String::from(""));

            let status = res.get_status();

            let status_line = format!("HTTP/1.1 {} {}\r\n", status, res.get_status_message());
            let content_type = format!("Content-Length: {}\r\n", res.get_json().len());
            let content_length = format!("Content-Type: {}\r\n", "application/json; charset=utf-8");
            let access_control = format!("access-control-allow-origin: {}\r\n\r\n", "*");
            let response = format!("{}{}{}{}{}", status_line, content_length, content_type, access_control, res.get_json());
    
            stream.write(response.as_bytes()).unwrap();
        
            stream.flush().unwrap();
        }
    }


    pub fn listen(self, port: i32) {
        let bind_to_address = format!("127.0.0.1:{}", port);

        let listener = TcpListener::bind(bind_to_address).unwrap();
        println!("Listening on port: {}", port);

        let server_arc = Arc::new(self);

        for stream in listener.incoming(){
            let stream = stream.unwrap();
        
            let server_arc_clone = Arc::clone(&server_arc);
            let pool = ThreadPool::new(4);
            
            pool.execute(move || {
                Server::handle_connection(stream, server_arc_clone);
            });
        }
    }

    pub fn close(&self){
        // drop(&self.pool);
    }
}

impl RouterService for Server {
    fn get(&mut self, path: &str, f: Controller) -> &mut Self{
        self.mount_router.get(&path, f);
        self
    }
  
    fn post(&mut self, path: &str, f: Controller) -> &mut Self{
        self.mount_router.post(&path, f);
        self
    }

    fn put(&mut self, path: &str, f: Controller) -> &mut Self{
        self.mount_router.put(&path, f);
        self
    }
  
    fn delete(&mut self, path: &str, f: Controller) -> &mut Self{
        self.mount_router.delete(&path, f);
        self
    }

    fn mount(&mut self, relative_path: &str, mut router: Router) -> &mut Self {
        let relative_path = String::from(relative_path);
        router.set_path(relative_path);
        self.mount_router.create_child_router(router);
        self
    }
}