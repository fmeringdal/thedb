use crate::request::Request;
use crate::response::Response;
use crate::router::{Controller, Middleware};

pub struct Route {
    pub path: String,
    pub method: String,
    handler: Controller
}

impl Route {
    pub fn new(path: String, f: Controller, method: String) -> Self {
        Route {
            path,
            handler: f,
            method
        }
    }

    pub fn handle(&self, req: &Request, res: &mut Response){
        &(self.handler)(req, res);
    }
}


pub struct MiddlewareRoute {
    pub path: String,
    pub method: String,
    handler: Middleware
}

impl MiddlewareRoute {
    pub fn new(path: String, f: Middleware, method: String) -> Self {
        MiddlewareRoute {
            path,
            handler: f,
            method
        }
    }

    pub fn handle(&self, req: &Request, res: &Response){
        &(self.handler)(req, res);
    }
}