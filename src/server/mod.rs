extern crate regex;
mod thread_pool;
mod response;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::net::{TcpStream, TcpListener};
use std::thread;
use self::response::Response;
use self::regex::Regex;


pub trait RouterAction: Send + Sync + 'static {
    fn call(&self, Response) -> ();
}

impl<T>RouterAction for T 
where T: Fn(Response) -> () + Send + Sync + 'static {
    fn call(&self, response: Response) -> () {
        (&self)(response);
    }
}

pub struct Server {
    routes: HashMap<String, HashMap<String, Box<RouterAction>>>,
}

impl Server {
    /// Creates a new instance of the server.
    pub fn new() -> Server {
        Server {
            routes: HashMap::new(),
        }
    }

    /// Creates a new route/path
    /// `route(HTTP_METHOD, PATH, ACTION)`
    ///
    /// * `HTTP_METHOD` as methods like; GET, PUT, POST, PATCH.
    /// * `PATH` as the route such as `/dogs`
    /// * `ACTION` as the closure/function that will be called on a successful route.
    ///
    /// *example*: `route("GET", "/dogs", dog_get)`
    pub fn route<T: RouterAction>(&mut self, method: String, path: String, action: T) {
        let mut method_storage = self.routes.entry(method).or_insert(HashMap::new());
        method_storage.insert(path, Box::new(action));
    }

    pub fn parse_incoming(&self, mut stream: &mut TcpStream) -> Result<Response, Error> {
        let mut response = Response::new(&mut stream)?;

        println!("{}", response.raw);
        println!("Response ended.");

        Ok(response)
    }

    /// Attaches the server to a port with an optional address (default loopback address IPV4)
    /// 
    /// # Panics if the post is closed or any other connection issue.
    pub fn listen(self, port: i16, address: Option<String>, threads: Option<usize>) {
        let address = address.unwrap_or(String::from("127.0.0.1"));
        let binding = TcpListener::bind(format!("{}:{}", address, port))
            .expect("Couldn't bind on port!");
        let pool = thread_pool::ThreadPool::new(threads.unwrap_or(4));
        let shared_self = Arc::new(self);

        for mut stream in binding.incoming() {
            let mut stream = match stream {
                Ok(v) => v,
                Err(_) => continue,  // TODO: Redirect to internal server error page.
            };

            let self_clone = shared_self.clone();
            thread::spawn(move || {
                self_clone.parse_incoming(&mut stream);
            });
        }
    }
}
