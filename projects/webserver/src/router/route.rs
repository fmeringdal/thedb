// use super::request::Request;
// use super::response::Response;
// use request::Request;
// use response::Response;

use crate::request::Request;
use crate::response::Response;

pub struct Route {
    pub path: String,
    pub method: String,
    pub handler: Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>
}

impl Route {
    // add code here
    pub fn match_route(&self, path: String, method: String) -> bool {
        if method == self.method {
            return true;
        }
        return false;
    }

    pub fn handler(&self, req: &Request, res: &mut Response){
        let handler = &self.handler;
        handler(req, res);
    }
}