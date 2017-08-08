extern crate regex;
mod thread_pool;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::net::{TcpStream, TcpListener};
use std::thread;
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

pub struct Response {
    route: String,
    action: String,
    headers: HashMap<String, String>,
    query: HashMap<String, String>,
    raw: Vec<u8>,
}

impl Response {
    pub fn new(stream: Vec<u8>) -> Result<Response, Error> {
        let stream_clone = stream.clone();

        let stream_as_string = String::from_utf8_lossy(&stream_clone);

        Ok(Response {
            route: String::from("hello"),
            action: String::from("hello"),
            headers: HashMap::new(),
            query: HashMap::new(),
            raw: stream,
        })
    }

    fn get_route(body: &str) -> String {
        "/get".to_string()
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

    pub fn parse_incoming(&self, stream: &mut TcpStream) -> Result<Response, Error> {
        let mut buf = vec![0];

        stream.read(&mut buf)?;
        let response = Response::new(buf)?;
        Ok(response)
    }

    pub fn listen(self, port: i16, address: Option<String>) {
        let address = address.unwrap_or(String::from("127.0.0.1"));
        let binding = TcpListener::bind(format!("{}:{}", address, port))
            .expect("Couldn't bind on port!");
        let pool = thread_pool::ThreadPool::new(8);
        let shared_self = Arc::new(self);

        for stream in binding.incoming() {
            let mut stream = match stream {
                Ok(v) => v,
                Err(_) => continue,   
            };

            let self_clone = shared_self.clone();
            thread::spawn(move || {
                self_clone.parse_incoming(&mut stream);
            });
        }
    }
}
