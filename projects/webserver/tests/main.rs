extern crate burner;

// use std::thread;
use burner::{Server, Request, Response, Router, RouterService};
// use std::time::Duration;
use serde_json::{json, Value};

mod tests {

    use super::*;

    #[test]
    fn dummy_test(){
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn port_test(){
        let mut server = Server::new();

        let handler = |_req: &Request, res: &mut Response| {
            let val = json!({ "tester": 2 });
            res.json(&val);
            res.status(200);
        };
    
        server.get("/health", Box::new(handler));
        server.get("/", Box::new(handler));

        server.listen(8989);
    }
}