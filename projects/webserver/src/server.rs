use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::router::{Router, Controller, RouterService, Middleware};
use crate::thread_pool::ThreadPool;
use crate::request::Request;
use crate::response::Response;

pub struct Server {
    mount_router: Router,
    thread_pool_size: usize
}

impl Server {
    pub fn new() -> Self {
        let mount_router = Router::new();

        return Server {
            thread_pool_size: 4,
            mount_router
        };
    }

    pub fn set_thread_pool_size(&mut self, size: usize){
        self.thread_pool_size = size;
    }
 
    fn create_response(mut stream: TcpStream, res: &Response){
        let response = res.get_http_response();
        stream.write(response.as_bytes()).unwrap(); 
        stream.flush().unwrap();
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let raw_request = String::from_utf8_lossy(&buffer);

        let mut res = Response::new();
        if let Ok(mut req) = Request::from(&raw_request) {
            self.mount_router.handle_request(&mut req, &mut res, &String::from(""));
        } else {
            println!("Unable to parse request from client");
            res.status(400);
        }

        Server::create_response(stream, &res);
    }


    pub fn listen(self, port: i32) {
        let bind_to_address = format!("127.0.0.1:{}", port);

        let listener = TcpListener::bind(bind_to_address)
            .expect(&format!("Expected {} to be available for Burner", port));
        println!("Burner is burning through requests on port: {}", port);

        let pool = ThreadPool::new(self.thread_pool_size);
        let server = Arc::new(self);

        for stream in listener.incoming(){
            let stream = match stream {
                Ok(stream) => stream,
                Err(err) => {
                    continue;
                }
            };

            let server = Arc::clone(&server);
            pool.execute(move || {
                server.handle_connection(stream);
            });
        }
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

    fn middleware(&mut self, path: &str, f: Middleware) -> &mut Self {
        self.mount_router.middleware(path, f);
        self
    }

    fn mount(&mut self, relative_path: &str, mut router: Router) -> &mut Self {
        let relative_path = String::from(relative_path);
        router.set_path(relative_path);
        self.mount_router.create_child_router(router);
        self
    }
}