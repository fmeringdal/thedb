extern crate burner;

// use std::thread;
use burner::{Server, Request, Response, Router, RouterService};
// use std::time::Duration;
use serde_json::{json, Value};

mod tests {

    use super::*;

    // #[test]
    fn dummy_test(){
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn port_test(){
        let mut server = Server::new();
        let mut user_router = Router::new();
        let mut post_router = Router::new();
        
        let get_user = |req: &Request, res: &mut Response| {
            println!("Get user controller");
            let response = format!("Hello user {}", req.route_params.get("userId").unwrap());
            res.send(&response);
        };
        
        let create_user = |req: &Request, res: &mut Response| {
            println!("Create user controller");
            let response: Value = json!({
                "working": true,
                "param": req.body["name"]
            });
            res.json(&response);
        };
        
        user_router
            .get("/:userId", Box::new(get_user)) // Will be accessible at path: /users/:userId
            .post("/", Box::new(create_user)); // Will be accessible at path: /users
        
        let get_post = |req: &Request, res: &mut Response| {
            println!("Get post controller");
        };
        
        let create_post = |req: &Request, res: &mut Response| {
            println!("Create post controller");
        };
        
        post_router
            .get("/:postId", Box::new(get_post)) // Will be accessible at path: /posts/:postId
            .post("/", Box::new(create_post)); // Will be accessible at path: /posts
        
        let user_router_path_prefix = "/users";
        let post_router_path_prefix = "/posts";
        
        // Mount routers into server (parent router)
        server
            .mount(user_router_path_prefix, user_router)
            .mount(post_router_path_prefix, post_router);
        // To create additional nesting just mount other routers on user_router or post_router or some other router
        
        // Start server
        let port = 6789;
        server.listen(port);
    }
}