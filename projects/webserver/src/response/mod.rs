pub struct Response {
    status: u32,
    json: String
} 

impl Response {
    pub fn new() -> Self {
        Response {
            status: 404,
            json: String::from("")
        }
    }

    pub fn status(&mut self, status: u32){
        self.status = status;
    }

    pub fn get_status(&self) -> u32 {
        return self.status;
    }

    pub fn send(&mut self, value: &str){
        self.json = String::from(value);
    }

    pub fn json(&mut self, value: &str){
        self.status = 200;
        self.json = String::from(value);
    }

    pub fn get_json(&mut self) -> &String {
        return &self.json;
    }
}