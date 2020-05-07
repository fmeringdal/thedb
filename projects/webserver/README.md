# Minimalistic REST server inspired by express.js

## Quick start
```rust
// Create the server
let mut server = Server::new();

// Create a route controller clojure
let controller = |req: &Request, res: &mut Response| { 
	// Set the HTTP status code to be 200 OK, default is 404
	res.status(200);
};

// Register controller on server to be triggered by a request to path / and method: GET
server.get(String::from("/"), Box::new(handler2));

// Start the server on port
let PORT = 6789;
server.listen(PORT);
```
## Route params
You can also have dynamic paths with route parameters in the path.
To specify a route parameters in the path use a colon followed by the name of the variable, e.g, ```/users/:userid```.
The parameter will be available on the request object with: ```req.route_params().get("userid")```
	

## Header variables
Header variables are also available on the request object. Tokens and other meta data are often store here. 
The parameter will be available on the request object with: ```req.headers.get("HEADER_KEY")```


## Returning data
```res.json("DATA_TO_RETURN")```
or
```res.send("DATA_TO_RETURN")```
