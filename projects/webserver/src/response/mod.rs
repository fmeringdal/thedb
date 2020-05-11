pub struct Response {
    status: u32,
    pub status_message: String,
    json: String
} 

impl Response {
    pub fn new() -> Self {
        Response {
            status: 404,
            status_message: String::from("Not Found"),
            json: String::from("")
        }
    }

    pub fn status(&mut self, status: u32){
        let (status_message, status) = match status {
            200 => ("Ok", status),
            404 => ("Not Found", status),
            _ => ("Not Found", 404),
        };
        
        self.status_message = String::from(status_message);
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