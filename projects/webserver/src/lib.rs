use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::mem::drop;

mod thread_pool;
use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, MutexGuard};

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
    handler: Box<dyn Fn() + Send + Sync + 'static>
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
    _routes: Arc<Mutex<Vec<Route>>>
}

pub struct ArcServer(Arc<Server>);

impl Server {
    pub fn new() -> Self {
        let pool = ThreadPool::new(4);
        let routes = vec![];
        
        return Server {
            pool: Arc::new(Mutex::new(pool)),
            _routes: Arc::new(Mutex::new(routes))
        };
    }

    pub fn get(&mut self, path: String, f: Box<dyn Fn() + Send + Sync + 'static> ){
        
        let method = String::from("get");

        let route = Route {
            path,
            handler: f,
            method
        };
        
        let mut routes = self._routes.lock().unwrap();
        routes.push(route);
    }

    fn handle_connection(mut stream: TcpStream, arc_server: Arc<Server>) {
        println!("inside handle connection");
        let mut buffer = [0; 512];
        println!("dinef buffer");
        println!("cloned arc");
        println!("Te routes lenght: {}", arc_server._routes.lock().unwrap().len());
        // let routes = arc_server._routes.lock().unwrap();
        // for route in routes {

        // }

        stream.read(&mut buffer).unwrap();
    
        println!("New request");
        let request = String::from_utf8_lossy(&buffer);
        println!("{}", request);
        let parsed: Vec<&str> = request.split_whitespace().collect();

        if parsed.len() >= 2 {
            let method = parsed[0];
            let path = parsed[1];
        }

    
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