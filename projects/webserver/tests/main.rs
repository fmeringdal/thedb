extern crate webserver;

use std::thread;
use webserver::{Server, Request, Response, Router, RouterService};
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

        let handler = |req: &Request, res: &mut Response| {
            println!("################# [NESTED TWICE] ################################");
            println!("Hello world from the the NESTED route handler!! :D");
            println!("{}", req.body["cools"]);
            res.json("Hello World");
        };

        server.post("/create", Box::new(handler));
        // server.listen(7878);
    }
}