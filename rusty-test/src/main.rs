extern crate rusty_server;
use std::collections::HashMap;
use rusty_server::server::Server;
use rusty_server::router::Router;
use rusty_server::request::Request;
use rusty_server::response::Response;

fn param_exists<T: ToString>(params: Option<HashMap<String, String>>, param: T) -> Option<String> {
    let param = param.to_string();
    match params {
        Some(p) => match p.get(&param) {
            Some(v) => return Some(v.to_string()),
            None => return None,
        },
        None => return None,
    }
}

fn main() {
    let mut server = Server::new();
    let mut router = Router::new();

    router.get("/", |req, mut res: Response| {
        res.send_file("./static/index.html");
    });

    router.get("/:id", |req: Request, mut res: Response| {
        match param_exists(req.params, "id") {
            Some(v) => res.send(format!("Hi!, You requested {}", v)),
            None => res.send("You didn't request anything!"),
        };
    });

    server.register("/", router);
    server.listen(3030, None, None);
}
