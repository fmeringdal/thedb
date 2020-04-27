use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

mod thread_pool;
use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

// ! TODO: Unwrap should be avoided at all costs, since a webserver should not be
// ! terminated because of a bad request

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, response) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "200 Ok")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "Woke up")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 Not found")
    };

    let response = format!("{}{}", status_line, response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

pub fn listen(port: i32){
    let bind_to_address = format!("127.0.0.1:{}", port);

    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind(bind_to_address).unwrap();

    println!("Listening on port: {}", port);

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

pub fn get_port() -> i32 {
    return 7878;
}

pub fn main(){
    let port = 6789;
    listen(port);
}