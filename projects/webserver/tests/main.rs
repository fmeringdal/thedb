extern crate burner;

// use std::thread;
use burner::{Server, Request, Response, Router, RouterService};
// use std::time::Duration;


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
            res.json("Hello World");
            res.status(200);
        };

        let mut router = Router::new();
        router.get("/esso", Box::new(handler));

        server
            .get("/health", Box::new(handler))
            .get("/anotherroute", Box::new(handler))
            .mount("/wassup", router);
    }
}