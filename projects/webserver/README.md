# Minimalistic REST server inspired by express.js

## Quick start
```rust
use webserver::{Server, Request, Response}; // Import into scope

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


## Returning data
```res.json("DATA_TO_RETURN")```
or
```res.send("DATA_TO_RETURN")```

## Nested routes
Nested routing is also possible. ```Router``` objects acts as building blocks and can be composed together by calling ```.mount``` on the parent Router with the child Router. Server extends (rust doesnt support inheritance, but ```Router``` and ```Server``` are implementing the same trait which is kind of similar to interfaces in other languages). So every method on ```Router``` is also available on ```Server```. Server will always act as the root router from which all requests will first be directed to.
An example app that leverages nested Routers.
```rust
use webserver::{Server, Request, Response, Router}; // Import into scope

// Initialize server and routers
let mut server = Server::new();
let mut user_router = Router::new();
let mut post_router = Router::new();

let getUser = |req: &Request, res: &mut Response| {
    println!("Get user controller");
};

let createUser = |req: &Request, res: &mut Response| {
    println!("Create user controller");
};

user_router.get("/:userId", Box::new(getUser)); // Will be accessible at path: /users/:userId
user_router.post("/", Box::new(createUser)); // Will be accessible at path: /users

let getPost = |req: &Request, res: &mut Response| {
    println!("Get post controller");
};

let createPost = |req: &Request, res: &mut Response| {
    println!("Create post controller");
};

post_router.get("/:postId", Box::new(getPost)); // Will be accessible at path: /posts/:postId
post_router.post("/", Box::new(createPost)); // Will be accessible at path: /posts

let user_router_path_prefix = "/users";
let post_router_path_prefix = "/posts";

// Mount routers into server (parent router)
server.mount(user_router_path_prefix, user_router); 
server.mount(post_router_path_prefix, post_router);
// To create additional nesting just mount other routers on user_router or post_router or some other router

// Start server
let port = 6789;
server.listen(port);
```



## Coming features
 - Middleware
