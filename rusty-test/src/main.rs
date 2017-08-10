extern crate rusty_mvc;
use rusty_mvc::server::Server;
use rusty_mvc::router::Router;

fn main() {
    let mut server = Server::new();
    let mut router = Router::new();

    router.route("GET", "/", |bep| {
        println!("Hello!");
    });

    router.get("/bep", |bep| {
        println!("Hi");
    });

    router.route("GET", "/", |bep| {
        println!("Hi");
    });

    router.get("/a", |test| {}).post("/ext", |test| {});

    server.register("/", router);

    server.listen(3030, None, None);
}
