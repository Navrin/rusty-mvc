pub mod router;
pub mod request;
mod thread_pool;

use std::net::{TcpListener, TcpStream};
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use self::request::Request;
use self::router::Router;

pub struct Server {
    inner_routers: Arc<Mutex<HashMap<String, Router>>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            inner_routers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers a new router for the server.
    pub fn register<T: ToString>(&mut self, path: T, router: Router) -> &mut Server {
        let inner = self.inner_routers.clone();

        let mut internal_router = inner.lock().expect("Could not lock routers!");
        internal_router.insert(path.to_string(), router);

        self
    }

    // Parsing!
    pub fn parse_incoming(&self, mut stream: &mut TcpStream) -> Result<Request, Error> {
        let request = Request::new(&mut stream)?;
        println!("{}", request);
        println!("request ended.");

        Ok(request)
    }

    /// Attaches the Router to a port with an optional address (default loopback address IPV4)
    ///
    /// # Panics if the post is closed or any other connection issue.
    pub fn listen(self, port: i16, address: Option<String>, threads: Option<usize>) {
        let address = address.unwrap_or(String::from("127.0.0.1"));
        let binding =
            TcpListener::bind(format!("{}:{}", address, port)).expect("Couldn't bind on port!");
        let pool = thread_pool::ThreadPool::new(threads.unwrap_or(4));
        let shared_self = Arc::new(self);

        for stream in binding.incoming() {
            let mut stream = match stream {
                Ok(v) => v,
                Err(e) => panic!(e),  // TODO: Redirect to internal Router error page.
            };

            let self_clone = shared_self.clone();
            pool.execute(move || { self_clone.parse_incoming(&mut stream); });
        }
    }
}
