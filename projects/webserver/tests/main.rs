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
            let var = req.route_params().get("testvar");
            res.status(200);
            res.send(String::from("Hello world from send!! :D"));
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

        server.get(String::from("/test"), Box::new(handler));
        server.get(String::from("/test/:testvar"), Box::new(handler2));
        server.post(String::from("/test"), Box::new(handler3));

        let mut router = Router::new();
        router.get( String::from("/nested"), Box::new(handler4));

        
        let mut nested_router = Router::new();
        nested_router.get(String::from("/nested2"), Box::new(handler5));
        
        router.nested( String::from("/nested"), nested_router);
        server.nested(String::from("/hallois"), router);


        server.listen(7878);
    }
}