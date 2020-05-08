extern crate serde_json;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::mem::drop;

mod thread_pool;
use thread_pool::ThreadPool;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

pub mod request;
pub mod response;

mod router;
pub use router::{Router};

pub use request::Request;
pub use response::Response;

pub struct Server {
    pool: Arc<Mutex<ThreadPool>>,
    mount_router: Router
}

pub struct ArcServer(Arc<Server>);

impl Server {
    pub fn new() -> Self {
        let pool = ThreadPool::new(4);
        let mount_router = Router::new();

        return Server {
            pool: Arc::new(Mutex::new(pool)),
            mount_router
        };
    }

    pub fn mount(&mut self, relative_path: &str, mut router: Router) {
        let relative_path = String::from(relative_path);
        router.set_path(relative_path);
        self.mount_router.create_child_router(router);
    }

    pub fn get(&mut self, path: &str, f: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static> ){
        self.mount_router.get(path, f);
    }

    pub fn post(&mut self, path: &str, f: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static> ){
        self.mount_router.post(&path, f);
    }

    fn handle_connection(mut stream: TcpStream, arc_server: Arc<Server>) {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();
    
        let request = String::from_utf8_lossy(&buffer);
        println!("New request [START]");
        println!("{}", request);
        println!("New request [DONE]");
        let parts: Vec<&str> = request.split("\n\r\n").collect();
        let mut request_headers: Vec<&str> = parts[0].split("\n").collect();
        request_headers.remove(0); // remove request line
        let mut headers: HashMap<String, String> = HashMap::new();

        for header in &request_headers {
            let header: Vec<&str> = header.split(": ").collect();
            headers.insert(String::from(header[0]), String::from(header[1]));
        }
        
        // let mut headers: Vec<&str> = parts[0].split("\n").collect();
        // headers.remove(0); // remove reuqest line


        // let message_body = parts[1];
        // println!("message_body: {}", message_body);

        // let val = json!(message_body);
        // println!("found body params: {}", val);

        let parsed: Vec<&str> = request.split_whitespace().collect();
        if parsed.len() < 2 {
            let response = format!("{}{}", 400, "Invalid request");

            stream.write(response.as_bytes()).unwrap();
        
            stream.flush().unwrap();
        } else {
    
            let method = String::from(parsed[0]);
            let path = String::from(parsed[1]);

            let mut req = Request::new(method, path);
            let mut res = Response::new();

            req.headers = headers;
            &arc_server.mount_router.handle_request(&mut req, &mut res, &String::from(""));

            let status = res.get_status();
            let status_line = format!("HTTP/1.1 {} OK\r\n\r\n", status);
            let response = format!("{}{}", status_line, res.get_json());
    
            stream.write(response.as_bytes()).unwrap();
        
            stream.flush().unwrap();
        }
    }


    pub fn listen(self, port: i32) {
        let bind_to_address = format!("127.0.0.1:{}", port);

        let listener = TcpListener::bind(bind_to_address).unwrap();
        println!("Listening on port: {}", port);

        let arc = Arc::new(self);

        for stream in listener.incoming(){
            let stream = stream.unwrap();

            let arc2 = Arc::clone(&arc);

            arc.pool.lock().unwrap().execute(move || {
                Server::handle_connection(stream, arc2);
            });
        }
    }

    pub fn close(&self){
        drop(&self.pool.lock().unwrap());
    }
}