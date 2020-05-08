extern crate webserver;

use std::thread;
use webserver::{Server, Request, Response, Router};
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
            println!("##########################################################################");
            println!("Hello world from the first route handler!! :D");
        };

        
        let handler2 = |req: &Request, res: &mut Response| {
            println!("##########################################################################");
            println!("Hello world from the seconf route handler!! :D");
            let var = req.route_params.get("testvar");
            res.status(200);
            let token = match req.headers.get("token") {
                Some(token) => token,
                None => "Does not exist",
            };

            res.send(String::from(format!("Token is: {}", token)));
            match var {
                Some(val) => println!("Hello variable: {}", val),
                None => println!("Did not find variable"),
            }
        };

        let handler3 = |req: &Request, res: &mut Response| {
            println!("##########################################################################");
            println!("Hello world from the the post route handler!! :D");
        };

        let handler4 = |req: &Request, res: &mut Response| {
            println!("################# [NESTED]               ################################");
            println!("Hello world from the the NESTED route handler!! :D");
        };

        let handler5 = |req: &Request, res: &mut Response| {
            println!("################# [NESTED TWICE] ################################");
            println!("Hello world from the the NESTED route handler!! :D");
        };

        server.get("/test", Box::new(handler));
        server.get("/test/:testvar", Box::new(handler2));
        server.post("/test", Box::new(handler3));

        let mut router = Router::new();
        router.get( "/nested", Box::new(handler4));

        
        let mut nested_router = Router::new();
        nested_router.get("/nested2", Box::new(handler5));
        
        router.mount( "/nested", nested_router);
        server.mount("/hallois", router);


        // server.listen(7878);
    }
}