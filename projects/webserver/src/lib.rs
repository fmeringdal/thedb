use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn listen(port: i32){
    let bind_to_address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(bind_to_address).unwrap();

    println!("Listening on port: {}", port);

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

pub fn get_port() -> i32 {
    return 7878;
}


