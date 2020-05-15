use crate::request::Request;
use crate::response::Response;
use crate::router::Controller;

pub struct Route {
    pub path: String,
    pub method: String,
    handler: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>
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