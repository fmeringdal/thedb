use crate::router::Route;

use crate::request::Request;
use crate::response::Response;

fn paths_match(route_path: &String, called_path: &String) -> bool {
    if *route_path == *called_path {
        return true;
    }

    let route_path_dir: Vec<&str> = route_path.split("/").collect();
    let called_path_dir: Vec<&str> = called_path.split("/").collect();
    if route_path_dir.len() != called_path_dir.len() {
        return false;
    }

    for i in 0..route_path_dir.len() {
        let nested_route_path = String::from(route_path_dir[i]);
        let nested_called_path = String::from(called_path_dir[i]);
        if nested_route_path.starts_with(":") {
           let route_param_name = &nested_route_path[1..];
           let route_param_value = nested_called_path;
        
        //    req.insert_route_param(String::from(route_param_name), route_param_value);
        } else {
            if nested_route_path != nested_called_path {
                return false;
            }
        }
    }

    return true;
}

pub struct Router {
    path: String,
    routes: Vec<Route>,
    childs: Vec<Router>
}

impl Router {
    pub fn new() -> Self {
        Router {
            path: String::from(""),
            routes: vec![],
            childs: vec![]
        }
    }

    fn create_route(&mut self, path: String, f: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>, method: String){
        let route = Route {
            path,
            handler: f,
            method
        };
        
        self.routes.push(route);
    }

    pub fn create_child_router(&mut self, router: Router){
        self.childs.push(router);
    }
    
    pub fn get(&mut self, path: String, f: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>){
      // add to route  
      let method = String::from("GET");
      self.create_route(path, f, method);
    }

    pub fn post(&mut self, path: String, f: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>){
        // add to route  
        let method = String::from("POST");
        self.create_route(path, f, method);
    }

    pub fn set_path(&mut self, path: String){
        println!("########################################################################################");
        println!("Setting path: {}", path);

        self.path = path;
    }

    // maybe use middleware instead of nested ?
    pub fn nested(&mut self, relative_path: String, mut router: Router) {
        // let path = format!("{}{}", self.path, relative_path);
        router.set_path(relative_path);
        self.childs.push(router);
    }

    pub fn handle_request(&self, req: &mut Request, res: &mut Response, parent_path: &String) -> bool {
        println!("Handle request for path: {}", req.path);
        // look through routes
        for route in &self.routes {
            println!("Checking child path: {}", route.path);
            let path = format!("{}{}{}", parent_path, self.path, route.path);
            println!("With path: {}", path);
            if route.method == req.method &&
            paths_match(&path, &req.path) {
                route.handler(req, res);
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