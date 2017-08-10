extern crate regex;

use std::collections::HashMap;
use std::string::ToString;
use server::request::Request;

#[derive(PartialEq, Eq, Hash, Clone)]
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
    fn call(&self, Request) -> ();
}

impl<T> RouterAction for T
where
    T: Fn(Request) -> () + Send + Sync + 'static,
{
    fn call(&self, request: Request) -> () {
        (&self)(request);
    }
}

pub struct Router {
    pub routes: HashMap<Methods, HashMap<String, Box<RouterAction>>>,
}

impl Router {
    /// Creates a new instance of the Router.
    pub fn new() -> Router {
        Router {
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
    pub fn route<T, P, M>(&mut self, method: M, path: P, action: T) -> &mut Router
    where
        T: RouterAction,
        P: ToString,
        M: RouterMethod,
    {
        let method = method.parse();
        self.routes
            .entry(method)
            .or_insert(HashMap::new())
            .insert(path.to_string(), Box::new(action));
        self
    }

    /// # Shorthand methods. .get instead of .route("GET")
    pub fn get<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Router {
        self.route(Methods::GET, path, action)
    }

    pub fn post<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Router {
        self.route(Methods::POST, path, action)
    }

    pub fn put<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Router {
        self.route(Methods::PUT, path, action)
    }

    pub fn patch<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Router {
        self.route(Methods::PATCH, path, action)
    }

    pub fn delete<T: RouterAction, S: ToString>(&mut self, path: S, action: T) -> &mut Router {
        self.route(Methods::DELETE, path, action)
    }
}
