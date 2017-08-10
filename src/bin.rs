extern crate rusty_mvc;
use rusty_mvc::server::Server;
mod server;

fn main() {
    let mut server = Server::new();
    
    server.route("GET", "/", | bep | {
        println!("Hello!");
    });

    server.get("/bep", | bep | {
        println!("Hi");
    });

    server.route("GET", "/", | bep | {
        println!("Hi");
    });

    server.get("/a", | test | {})
          .post("/ext", | test | {});

    server.listen(3030, None, None);
}