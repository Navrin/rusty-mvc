extern crate rusty_server;
use rusty_server::server::Server;
use rusty_server::router::Router;
use rusty_server::request::Request;
use rusty_server::response::Response;

fn main() {
    let mut server = Server::new();
    let mut router = Router::new();
    let mut second_router = Router::new();
    router.get("/bep", |req, res| {
        println!("Hi");
    });

    router.get("/", |req, mut res: Response| {
        res.send("Hello, this is a test!");
    });

    router.get("/a", |test, res| {}).post("/ext", |req, res| {});

    router.get("/dogs", |req, res| {
        println!("WOHOO DOGS");
    });

    router.get("/cats", |req, res| {
        println!("Cats are alright too");
    });

    router.get("/dogs/:id", |req: Request, res| {
        println!("{}", req.params.unwrap().get("id").unwrap());
    });

    second_router.get("/a-mystery", |req, res| {
        println!("Hi!");
    });

    server.register("/", router);
    server.register("/inner", second_router);

    server.listen(3030, None, None);
}
