extern crate regex;
mod thread_pool;
mod response;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::string::ToString;
use self::response::Response;
use self::regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Methods {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    ALL,
}

pub trait RouterMethod {
    fn parse(&self) -> Methods;
}

impl<T: ToString> RouterMethod for T {
    fn parse(&self) -> Methods {
        match self.to_string().to_uppercase().as_ref() {
            "GET" => Methods::GET,
            "POST" => Methods::POST,
            "PUT" => Methods::PUT,
            "DELETE" => Methods::DELETE,
            "PATCH" => Methods::PATCH,
            "*" => Methods::ALL,
            _ => panic!(format!("{} is not a (supported) method!", self.to_string())),
        }
    }
}

impl RouterMethod for Methods {
    fn parse(&self) -> Methods {
        let en = self.clone();
        en
    }
}

pub trait RouterAction: Send + Sync + 'static {
    fn call(&self, Response) -> ();
}

impl<T> RouterAction for T
where
    T: Fn(Response) -> () + Send + Sync + 'static,
{
    fn call(&self, response: Response) -> () {
        (&self)(response);
    }
}

pub struct InnerServer {
    pub routes: HashMap<Methods, HashMap<String, Box<RouterAction>>>,
}

pub struct Server {
    inner: Arc<InnerServer>,
}

impl Server {
    /// Creates a new instance of the server.
    pub fn new() -> Server {
        Server {
            inner: Arc::new(InnerServer {
                routes: HashMap::new(),
            }),
        }
    }

    fn mut_inner(&mut self) -> &mut InnerServer {
        Arc::get_mut(&mut self.inner).expect("Could not modify router")
    }

    /// Creates a new route/path
    /// `route(HTTP_METHOD, PATH, ACTION)`
    ///
    /// * `HTTP_METHOD` as methods like; GET, PUT, POST, PATCH.
    /// * `PATH` as the route such as `/dogs`
    /// * `ACTION` as the closure/function that will be called on a successful route.
    ///
    /// *example*: `route("GET", "/dogs", dog_get)`
    pub fn route<T, P, M>(&mut self, method: M, path: P, action: T) -> &mut Server
    where
        T: RouterAction,
        P: ToString,
        M: RouterMethod,
    {
        let method = method.parse();
        let mut method_storage = self.mut_inner()
            .routes
            .entry(method)
            .or_insert(HashMap::new())
            .insert(path.to_string(), Box::new(action));
        self
    }

    /// # Shorthand methods. .get instead of .route("GET")
    pub fn get<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Server {
        self.route(Methods::GET, path, action)
    }

    pub fn post<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Server {
        self.route(Methods::POST, path, action)
    }

    pub fn put<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Server {
        self.route(Methods::PUT, path, action)
    }

    pub fn patch<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Server {
        self.route(Methods::PATCH, path, action)
    }

    pub fn delete<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Server {
        self.route(Methods::DELETE, path, action)
    }
    
    // Parsing!

    pub fn parse_incoming(&self, mut stream: &mut TcpStream) -> Result<Response, Error> {
        let mut response = Response::new(&mut stream)?;

        println!("{:?}", response.raw);
        println!("Response ended.");

        Ok(response)
    }

    /// Attaches the server to a port with an optional address (default loopback address IPV4)
    ///
    /// # Panics if the post is closed or any other connection issue.
    pub fn listen(self, port: i16, address: Option<String>, threads: Option<usize>) {
        let address = address.unwrap_or(String::from("127.0.0.1"));
        let binding =
            TcpListener::bind(format!("{}:{}", address, port)).expect("Couldn't bind on port!");
        let pool = thread_pool::ThreadPool::new(threads.unwrap_or(4));
        let shared_self = Arc::new(self);

        for mut stream in binding.incoming() {
            let mut stream = match stream {
                Ok(v) => v,
                Err(e) => panic!(e),  // TODO: Redirect to internal server error page.
            };

            let self_clone = shared_self.clone();
            thread::spawn(move || { self_clone.parse_incoming(&mut stream); });
        }
    }
}
