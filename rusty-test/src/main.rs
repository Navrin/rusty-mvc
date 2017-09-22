extern crate rusty_server;
use std::collections::HashMap;
use rusty_server::server::Server;
use rusty_server::router::Router;
use rusty_server::request::Request;
use rusty_server::response::Response;
use rusty_server::middleware::{MiddlewareSession};
use rusty_server::session::Session;

fn main() {
    let mut server = Server::new();
    let mut router = Router::new();
    router.get("/", |req: &Request,  res: &mut Response, mut session: MiddlewareSession | {
        res.send_file("./static/index.html");
        session.terminate();
    });

    router.get("/hi", Session::new()
        .then(| req: &Request, res: &mut Response, mut session: MiddlewareSession| {
            println!("Previous!");
            session.next();
        })
        .then(| req: &Request, res: &mut Response, mut session: MiddlewareSession | {
            res.send("Hello");
            session.terminate();
        })
    );

    // router.get("/:id", |req: &Request, mut res: Response| {
    //     match req.find_query("id") {
    //         Some(v) => res.send(format!("Hi!, You requested {}", v)),
    //         None => res.send("You didn't request anything!"),
    //     };
    // });

    server.register("/", router);
    server.listen(3030, None, None);
}
