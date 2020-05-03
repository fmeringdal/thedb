extern crate webserver;

use std::thread;
use webserver::Server;
use std::time::Duration;


mod tests {

    use super::*;

    #[test]
    fn dummy_test(){
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn port_test(){
        let mut server = Server::new();

        let handler = || {
            println!("##########################################################################");
            println!("Hello world from the first route handler!! :D");
        };

        
        let handler2 = || {
            println!("##########################################################################");
            println!("Hello world from the seconf route handler!! :D");
        };

        server.get(String::from("/test"), Box::new(handler));
        server.get(String::from("/test/:23124"), Box::new(handler2));

        server.listen(7878);
        // server.close();

    }
}