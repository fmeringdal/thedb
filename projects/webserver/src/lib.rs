extern crate serde_json;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::mem::drop;
use serde_json::json;

mod thread_pool;
use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, MutexGuard};



mod request;
mod response;

pub use request::Request;
pub use response::Response;

// pub struct Request;
// pub struct Response;

// ! TODO: Unwrap should be avoided at all costs, since a webserver should not be
// ! terminated because of a bad request

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 512];

//     stream.read(&mut buffer).unwrap();
    
//     let get = b"GET / HTTP/1.1\r\n";
//     let sleep = b"GET /sleep HTTP/1.1\r\n";

//     let (status_line, response) = if buffer.starts_with(get){
//         ("HTTP/1.1 200 OK\r\n\r\n", "200 Ok")
//     } else if buffer.starts_with(sleep) {
//         thread::sleep(Duration::from_secs(5));
//         ("HTTP/1.1 200 OK\r\n\r\n", "Woke up")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 Not found")
//     };

//     let response = format!("{}{}", status_line, response);

//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }


struct Route {
    path: String,
    method: String,
    handler: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>
}



impl Route {
    // add code here
    pub fn match_route(&self, path: String, method: String) -> bool {
        if(method == self.method){
            return true;
        }
        return false;
    }
}


pub struct Server {
    pool: Arc<Mutex<ThreadPool>>,
    _routes: Vec<Route>
}

pub struct ArcServer(Arc<Server>);

fn paths_match(route_path: &String, called_path: &String, req: &mut Request) -> bool {
    if *route_path == *called_path {
        return true;
    }

    let route_path_dir: Vec<&str> = route_path.split("/").collect();
    let called_path_dir: Vec<&str> = called_path.split("/").collect();
    if route_path_dir.len() != called_path_dir.len() {
        return false;
    }

    for i in 0..route_path_dir.len() {
        let nested_route_path = String::from(route_path_dir[i]);
        let nested_called_path = String::from(called_path_dir[i]);
        if nested_route_path.starts_with(":") {
           let route_param_name = &nested_route_path[1..];
           let route_param_value = nested_called_path;
        
           req.insert_route_param(String::from(route_param_name), route_param_value);
        } else {
            if nested_route_path != nested_called_path {
                return false;
            }
        }
    }

    return true;
}

impl Server {
    pub fn new() -> Self {
        let pool = ThreadPool::new(4);
        let routes = vec![];
        
        return Server {
            pool: Arc::new(Mutex::new(pool)),
            _routes: routes
        };
    }

    fn add_route(&mut self, path: String, f: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>, method: String){
        let route = Route {
            path,
            handler: f,
            method
        };
        
        self._routes.push(route);
    }

    pub fn get(&mut self, path: String, f: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static> ){
        let method = String::from("GET");
        self.add_route(path, f, method);
    }

    pub fn post(&mut self, path: String, f: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static> ){
        let method = String::from("POST");
        self.add_route(path, f, method);
    }

    fn handle_connection(mut stream: TcpStream, arc_server: Arc<Server>) {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();
    
        let request = String::from_utf8_lossy(&buffer);
        println!("New request [START]");
        println!("{}", request);
        println!("New request [DONE]");
        let parts: Vec<&str> = request.split("\n\r\n").collect();
        if parts.len() != 2 {
            println!("Incorrect HTTP");
            return;
        }
        // for part in &parts {
        //     println!("part: {}", part);
        //     println!("heilength: {}", part.len());
        //     if *part == "\r"  {
        //         println!("just empty ...")
        //     }
        // }
        let mut headers: Vec<&str> = parts[0].split("\n").collect();
        headers.remove(0); // remove reuqest line


        let message_body = parts[1];
        println!("message_body: {}", message_body);

        // let val = json!(message_body);
        // println!("found body params: {}", val);

        let parsed: Vec<&str> = request.split_whitespace().collect();
        
        let mut req = Request::new();
        let mut res = Response::new();

        if parsed.len() >= 2 {
            let method = parsed[0];
            let path = String::from(parsed[1]);
            for route in &arc_server._routes {
                if route.method == method &&
                paths_match(&route.path, &path, &mut req) {
                   let handler = &route.handler;
                   handler(&req, &mut res);
                   break;
               }
            }
            // for i in 0..arc_server._routes.len() {
            //     if arc_server._routes[i].method == method &&
            //      paths_match(&arc_server._routes[i].path, &path, &mut req) {
            //         let handler = &arc_server._routes[i].handler;
            //         handler(&req, &mut res);
            //         break;
            //     }
            // }
        }

        let status = res.get_status();
        let status_line = format!("HTTP/1.1 {} OK\r\n\r\n", status);
        let response = format!("{}{}", status_line, res.get_json());

        stream.write(response.as_bytes()).unwrap();
    
        stream.flush().unwrap();
    }


    pub fn listen(self, port: i32) -> i32 {
        let bind_to_address = format!("127.0.0.1:{}", port);

    
        let listener = TcpListener::bind(bind_to_address).unwrap();

    
        println!("Listening on port: {}", port);

        let arc = Arc::new(self);

        // let routes = self._routes.lock().unwrap();

        for stream in listener.incoming(){
            let stream = stream.unwrap();

            let arc2 = Arc::clone(&arc);

            arc.pool.lock().unwrap().execute(move || {
                Server::handle_connection(stream, arc2);
            });
        }
        port
    }

    pub fn close(&self){
        drop(&self.pool);
    }
}