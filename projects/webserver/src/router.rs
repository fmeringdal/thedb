use crate::route::{Route, MiddlewareRoute};
use crate::request::Request;
use crate::response::Response;
use serde_json::Value;

use std::collections::HashMap;

pub type Controller = Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>;
pub type Middleware = Box<dyn Fn(&Request, &Response) + Send + Sync + 'static>;

pub trait RouterService {
    // For registration of route controllers
    fn get(&mut self, path: &str, f: Controller) -> &mut Self;
    fn post(&mut self, path: &str, f: Controller)-> &mut Self;
    fn put(&mut self, path: &str, f: Controller)-> &mut Self;
    fn delete(&mut self, path: &str, f: Controller)-> &mut Self;
    
    // middleware
    fn middleware(&mut self, path: &str, f: Middleware)-> &mut Self;

    // For mounting nested RouterServices
    fn mount(&mut self, relative_path: &str, router: Router) -> &mut Self;
}


fn paths_match(route_path: &String, req: &mut Request) -> bool {
    // !! Hack
    let called_path = &req.path;

    if *route_path == *called_path {
        return true;
    }

    let route_path_dir: Vec<&str> = route_path.split("/").collect();
    let called_path_dir: Vec<&str> = called_path.split("/").collect();
    if route_path_dir.len() != called_path_dir.len() {
        return false;
    }

    let mut route_params = HashMap::new();
    for i in 0..route_path_dir.len() {
        let mut nested_route_path = String::from(route_path_dir[i]);
        let mut nested_called_path = String::from(called_path_dir[i]);
        if nested_route_path.starts_with(":") {
           let route_param_name = &nested_route_path[1..];
           let route_param_value = nested_called_path;
           route_params.insert(String::from(route_param_name), route_param_value);
        } else {
            if nested_route_path.ends_with("/") {
                nested_route_path.truncate(nested_route_path.len() - 1);
            }
            if nested_called_path.ends_with("/") {
                nested_called_path.truncate(nested_called_path.len() - 1);
            }
            if nested_route_path != nested_called_path {
                return false;
            }
        }
    }
    
    // When paths match then insert route params
    for (name, value) in route_params {
        req.route_params.insert(name, value);
    }

    return true;
}

fn collect_route_params(route_path: &String, called_path: &String) -> HashMap<String, String> {

    let route_path_dir: Vec<&str> = route_path.split("/").collect();
    let called_path_dir: Vec<&str> = called_path.split("/").collect();

    let mut params = HashMap::new();

    for i in 0..route_path_dir.len() {
        let nested_route_path = String::from(route_path_dir[i]);
        let nested_called_path = String::from(called_path_dir[i]);
        if nested_route_path.starts_with(":") {
           let route_param_name = &nested_route_path[1..];
           let route_param_value = nested_called_path;
            params.insert(String::from(route_param_name), route_param_value);
        }
    }

    return params;
}

fn path_is_subpath(route_path: &String, called_path: &String) -> bool {
    if *route_path == *called_path || route_path == "" {
        return true;
    }

    let route_path_dir: Vec<&str> = route_path.split("/").collect();
    let called_path_dir: Vec<&str> = called_path.split("/").collect();
    if route_path_dir.len() > called_path_dir.len() {
        return false;
    }

    for i in 0..route_path_dir.len() {
        let nested_route_path = String::from(route_path_dir[i]);
        let nested_called_path = String::from(called_path_dir[i]);
        if !nested_route_path.starts_with(":") && nested_route_path != nested_called_path {
           return false;
        }
    }

    return true;
}

pub struct Router {
    path: String,
    routes: Vec<Route>,
    middleware: Vec<MiddlewareRoute>,
    childs: Vec<Router>
}

impl Router {
    pub fn new() -> Self {
        Router {
            path: String::from(""),
            routes: vec![],
            middleware: vec![],
            childs: vec![]
        }
    }

    fn create_route(&mut self, path: String, f: Controller, method: String){
        let route = Route::new(path, f, method);
        self.routes.push(route);
    }

    fn create_middleware_route(&mut self, path: String, f: Middleware, method: String){
        let route = MiddlewareRoute::new(path, f, method);
        self.middleware.push(route);
    }

    pub fn create_child_router(&mut self, router: Router){
        self.childs.push(router);
    }
    
    pub fn set_path(&mut self, path: String){
        self.path = path;
    }

    pub fn handle_request(&self, mut req: &mut Request, res: &mut Response, parent_path: &String) -> bool {
        let router_path = format!("{}{}", parent_path, self.path);
        if !path_is_subpath(&router_path, &req.path) {
            return false;
        }

        // look through middleware
        for route in &self.routes {
            let path = format!("{}{}", router_path, route.path);
            if paths_match(&path, &mut req) {
                route.handle(req, res);
            }
        }

        // look through routes
        for route in &self.routes {
            let path = format!("{}{}", router_path, route.path);
            if route.method == req.method &&
            paths_match(&path, &mut req) {
                let route_params = collect_route_params(&path, &req.path);
                req.route_params = route_params;
                route.handle(req, res);
                return true;
            }
        }

        // look through childs
        for child_router in &self.childs {
            if child_router.handle_request(req, res, &self.path) {
                return true;
            }
        }

        return false;
    }
}

impl RouterService for Router {
    fn get(&mut self, path: &str, f: Controller) -> &mut Self {
        let method = String::from("GET");
        let path = String::from(path);
        self.create_route(path, f, method);
        self
    }
  
    fn post(&mut self, path: &str, f: Controller) -> &mut Self {
        let method = String::from("POST");
        let path = String::from(path);
        self.create_route(path, f, method);
        self
    }

    fn put(&mut self, path: &str, f: Controller) -> &mut Self {
        let method = String::from("PUT");
        let path = String::from(path);
        self.create_route(path, f, method);
        self
    }
  
    fn delete(&mut self, path: &str, f: Controller) -> &mut Self {
        let method = String::from("DELETE");
        let path = String::from(path);
        self.create_route(path, f, method);
        self
    }

    fn middleware(&mut self, path: &str, f: Middleware) -> &mut Self {
        let method = String::from("*");
        let path = String::from(path);
        self.create_middleware_route(path, f, method);
        self
    }

    fn mount(&mut self, relative_path: &str, mut router: Router)-> &mut Self {
        let relative_path = String::from(relative_path);
        router.set_path(relative_path);
        self.childs.push(router);
        self
    }
}