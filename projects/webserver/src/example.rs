use webserver;

let webserver = webserver::new();

webserver.get("/:test/hello")

webserver.post("/hello")

webserver.put()

webserver.listen(6868)
