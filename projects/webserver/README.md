# Minimalistic REST server inspired by express.js

## Quick start
```rust
extern crate burner;

use burner::{Server, Request, Response, RouterService}; // Import into scope

// Create the server
let mut server = Server::new();

// Create a route controller clojure
let controller = |req: &Request, res: &mut Response| { 
	// Set the HTTP status code to be 200 OK, default is 404
	res.status(200);
};

// Register controller on server to be triggered by a request to path / and method: GET
let path = "/";
server.get(path, Box::new(controller));

// Start the server on port
let PORT = 6789;
server.listen(PORT);
```
## Route params
You can also have dynamic paths with route parameters in the path.
To specify a route parameters in the path use a colon followed by the name of the variable, e.g, ```/users/:userid```.
The parameter will be available on the request object with: ```req.route_params.get("userid")```
	

## Header variables
Header variables are also available on the request object. Tokens and other meta data are often store here. 
The parameter will be available on the request object with: ```req.headers.get("HEADER_KEY")```


## Request Body
Request Body can be accessed by ```request.body```, e.g: ```let user_name: &str = req.body['name']``` 


## Returning data
```res.json(json: &Value)``` for json data using ```serde_json::Value```
or
```res.send(msg: &str)``` for sending string messages.
```res.json``` and ```res.send```  sets the status code to ```200``` automatically. To override it call
```res.status(status: u32)``` after ```res.send``` or ```res.json```. 

## Nested routes
Nested routing is also possible. ```Router``` objects acts as building blocks and can be composed together by calling ```.mount``` on the parent Router with the child Router. Server extends (rust doesnt support inheritance, but ```Router``` and ```Server``` are implementing the same trait which is kind of similar to interfaces in other languages). So every method on ```Router``` is also available on ```Server```. Server will always act as the root router from which all requests will first be directed to.
An example app that leverages nested Routers.
```rust
extern crate burner;

use burner::{Server, Request, Response, Router, RouterService}; // Import into scope

// Initialize server and routers
let mut server = Server::new();
let mut user_router = Router::new();
let mut post_router = Router::new();

let get_user = |req: &Request, res: &mut Response| {
    println!("Get user controller");
    let user_id = req.route_params.get("userid").unwrap();
    let response = format!("Hello user {}", user_id);
    res.send(&response);
};

let create_user = |req: &Request, res: &mut Response| {
    res.send("Create user controller");
};

user_router
    .get("/:userid", Box::new(get_user)) // Will be accessible at path: /users/:userId
    .post("/", Box::new(create_user)); // Will be accessible at path: /users

let get_post = |req: &Request, res: &mut Response| {
    res.send("Get post controller");
};

let create_post = |req: &Request, res: &mut Response| {
    res.send("Create post controller");
};

post_router
    .get("/:postid", Box::new(get_post)) // Will be accessible at path: /posts/:postId
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
```



## Coming features
 - Middleware
