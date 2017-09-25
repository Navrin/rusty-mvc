use std::collections::HashMap;
use std::iter::Iterator;
use std::string::ToString;
use std::cell::RefCell;
use std::io::{Error, ErrorKind};
use std::sync::{Arc, RwLock};
use server::request::Request;
use server::response::Response;
use server::session::{Session, Sessionable};
use server::middleware::{MiddlewareMethod, MiddlewareSession};

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

// pub trait RouterAction: Send + Sync + 'static {
//     fn call(&self, &Request, &mut Response) -> ();
// }

// impl<T> RouterAction for T
// where
//     T: Fn(&Request, &mut Response) -> () + Send + Sync + 'static,
// {
//     fn call(&self, request: &Request, response: &mut Response) -> () {
//         (*self)(request, response);
//     }
// }

pub struct InnerRouter {
    pub routes: HashMap<Methods, HashMap<String, Arc<RwLock<Session>>>>,
    pub middlewares: Arc<RwLock<Option<Session>>>,
}

pub struct Router {
    pub inner: Arc<RwLock<InnerRouter>>,
}

impl Router {
    /// Creates a new instance of the Router.
    pub fn new() -> Router {
        Router {
            inner: Arc::new(RwLock::new(InnerRouter {
                routes: HashMap::new(),
                middlewares: Arc::new(RwLock::new(None)),
            })),
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
    pub fn route<S, P, M>(&mut self, method: M, path: P, action: S) -> &mut Router
    where
        S: Sessionable,
        P: ToString,
        M: RouterMethod,
    {
        let mut self_mutex = self.inner.clone();
        let mut self_ref = self_mutex.write().unwrap();
        let mut routes = self_ref
            .routes
            .entry(method.parse())
            .or_insert(HashMap::new());
        let set = routes.insert(path.to_string(), Arc::new(RwLock::new(action.parse())));

        self
    }

    // pub fn register<T: MiddlewareMethod>(&mut self, middleware: T) -> &mut Router {
    //     let middlewares = self.middlewares.clone();
    //     let mut middlewares_ref = middlewares.try_lock().unwrap();
    //     middlewares_ref.push(Box::new(middleware));
    //     self
    // }


    /// Searches the routers for the correct path, finding the action for the path.
    /// It also finds params within the url, like `dog/:id/`.
    pub fn find_route(
        &self,
        method: String,
        path: String,
    ) -> Result<(Arc<RwLock<Session>>, HashMap<String, String>), Error> {
        let self_rw = self.inner.clone();
        let self_ref = self_rw.read().unwrap();

        let routes = self_ref.routes.get(&method.parse());

        let routes = match routes {
            Some(v) => v,
            None => return Err(Error::new(ErrorKind::NotFound, "404")),
        };

        let split_path = path.to_string();
        let split_path = split_path.split('/').collect::<Vec<&str>>();

        for (route, method) in routes.iter() {
            let template = route.split('/').collect::<Vec<&str>>();
            let mut params: HashMap<String, String> = HashMap::new();

            if template
                .iter()
                .zip(&split_path)
                .all(|(templ_seg, path_seg)| {
                    if templ_seg.contains(':') {
                        params.insert(
                            templ_seg.trim_matches(':').to_string(),
                            path_seg.to_string(),
                        );
                        return true;
                    }
                    templ_seg == path_seg
                }) {
                let method_copy = method.clone();

                return Ok((method_copy, params));
            }
        }

        Err(Error::new(ErrorKind::NotFound, "404"))
    }

    /// # Shorthand methods. .get instead of .route("GET")
    pub fn get<T: MiddlewareMethod, E: Sessionable, S: ToString>(
        &mut self,
        path: S,
        action: E,
    ) -> &mut Router {
        self.route(Methods::GET, path, action)
    }

    pub fn post<T: MiddlewareMethod, E: Sessionable, S: ToString>(
        &mut self,
        path: S,
        action: E,
    ) -> &mut Router {
        self.route(Methods::POST, path, action)
    }

    pub fn put<T: MiddlewareMethod, E: Sessionable, S: ToString>(
        &mut self,
        path: S,
        action: E,
    ) -> &mut Router {
        self.route(Methods::PUT, path, action)
    }

    pub fn patch<T: MiddlewareMethod, E: Sessionable, S: ToString>(
        &mut self,
        path: S,
        action: E,
    ) -> &mut Router {
        self.route(Methods::PATCH, path, action)
    }

    pub fn delete<T: MiddlewareMethod, E: Sessionable, S: ToString>(
        &mut self,
        path: S,
        action: E,
    ) -> &mut Router {
        self.route(Methods::DELETE, path, action)
    }
}

#[test]
fn test() {
    let mut router = Router::new();
    router.get(
        "/",
        |req: &Request, res: &mut Response, mut session: MiddlewareSession| {
            res.send_file("./static/index.html");
            session.terminate();
        },
    );

    // router.get(
    //     "/hi",
    //     Session::new().then(
    //         |req: &Request, res: &mut Response, mut session: MiddlewareSession| {
    //             res.send_file("./static/index.html");
    //             session.terminate();
    //         },
    //     ),
    // );
}
