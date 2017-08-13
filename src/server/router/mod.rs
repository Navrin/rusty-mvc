use std::collections::HashMap;
use std::string::ToString;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use server::request::Request;
use server::response::Response;

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
    fn call(&self, Request, Response) -> ();
}

impl<T> RouterAction for T
where
    T: Fn(Request, Response) -> () + Send + Sync + 'static,
{
    fn call(&self, request: Request, response: Response) -> () {
        (&self)(request, response);
    }
}

pub struct Router {
    pub routes: HashMap<Methods, HashMap<String, Arc<RouterAction>>>,
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
        self.routes
            .entry(method.parse())
            .or_insert(HashMap::new())
            .insert(path.to_string(), Arc::new(action));
        self
    }


    /// Searches the routers for the correct path, finding the action for the path.
    /// It also finds params within the url, like `dog/:id/`.
    pub fn find_route(
        &self,
        method: String,
        path: String,
    ) -> Result<(Arc<RouterAction>, HashMap<String, String>), Error> {
        let routes = self.routes.get(&method.parse());

        let routes = match routes {
            Some(v) => v,
            None => return Err(Error::new(ErrorKind::NotFound, "404")),
        };

        let split_path = path.to_string();
        let split_path = split_path.split('/').collect::<Vec<&str>>();

        for (route, method) in routes.iter() {
            let template = route.split('/').collect::<Vec<&str>>();
            let mut params: HashMap<String, String> = HashMap::new();

            if template.iter().zip(&split_path).all(
                |(templ_seg, path_seg)| {
                    if templ_seg.contains(':') {
                        params.insert(
                            templ_seg.trim_matches(':').to_string(),
                            path_seg.to_string(),
                        );
                        return true;
                    }
                    templ_seg == path_seg
                },
            ) {
                let method_copy = method.clone();
                return Ok((method_copy, params));
            }
        }

        Err(Error::new(ErrorKind::NotFound, "404"))
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
