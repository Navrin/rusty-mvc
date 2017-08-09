extern crate rusty_mvc;
use rusty_mvc::server::Server;
mod server;

fn main() {
    let mut server = Server::new();
    
    server.route("GET".to_string(), "/".to_string(), | bep | {
        println!("Hello!");
    });

    server.listen(3030, None, None);
}