pub mod router;
pub mod request;
pub mod response;
pub mod middleware;
pub mod session;
mod thread_pool;

use std::net::{TcpListener, TcpStream};
use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::rc::Rc;
use std::cell::RefCell;

use self::request::Request;
use self::router::Router;
use self::response::Response;
use self::middleware::{MiddlewareMethod, MiddlewareSession};
use self::session::Session;

pub struct ServerInner<T: MiddlewareMethod> {
    inner_routers: Mutex<HashMap<String, Router<T>>>,
}

pub struct Server<T: MiddlewareMethod> {
    inner: Arc<ServerInner<T>>,
}

impl<T: MiddlewareMethod> Server<T> {
    pub fn new() -> Server<T> {
        Server {
            inner: Arc::new(ServerInner {
                inner_routers: Mutex::new(HashMap::new()),
            }),
        }
    }

    /// Registers a new router for the server.
    pub fn register<S: ToString>(&mut self, path: S, router: Router<T>) -> &mut Server<T> {
        let inner = self.inner.clone();
        let mut routers = inner.inner_routers.lock().expect("Could not lock!");
        let empty_path = "/".to_string();

        let path = if path.to_string() == empty_path {
            "".to_string()
        } else {
            path.to_string()
        };

        routers.insert(path, router);

        self
    }

    // Parsing!
    pub fn parse_incoming(&self, mut stream: &mut TcpStream) -> Result<(), Error> {
        let mut request = Request::new(&mut stream)?;
        // let wares = self.find_middlewares(&request.route);

        let (path_wares, params) = self.find_route(&request.method, &request.route)?;
        if params.len() > 0 {
            request.params = Some(params);
        }

        let stream_copy = stream.try_clone().unwrap();
        let mut response = Response::new(stream_copy);

        // let session = path_wares.clone().try_lock().unwrap();
        // let wares = session.wares.clone();

        // for middleware in wares {
        //         // let req = request_rc.clone();
        //         // let res = response_rc.clone();
        //         let (send, revc) = channel::<bool>();
        //         let session = MiddlewareSession::new(send);

        //         middleware.call(&request , &mut response, session);
        // }

        Ok(())
    }

    fn find_middlewares(&self, path: &String) -> Option<Arc<Mutex<Option<Session<T>>>>> {
        let inner = self.inner.clone();
        let inner = inner.inner_routers.lock();
        let routers = match inner {
            Ok(v) => v,
            _ => return None,
        };
        let routers = routers.iter();

        for (routing, router) in routers {
            let routing = routing.to_string();
            if path.trim_left().starts_with(&routing) {
                let middlewares = router.middlewares.clone();
                return Some(middlewares);
            }
        }

        return None;
    }

    // finds the specified route's action
    pub fn find_route(
        &self,
        method: &String,
        path: &String,
    ) -> Result<(Arc<Mutex<Session<T>>>, HashMap<String, String>), Error> {
        let inner = self.inner.clone();
        let inner = inner.inner_routers.lock();
        let routers = match inner {
            Ok(v) => v,
            _ => return Err(Error::new(ErrorKind::NotFound, "404")),
        };

        let routers = routers.iter();

        for (routing, router) in routers {
            let routing = routing.to_string();
            if path.trim_left().starts_with(&routing) {
                let (method, params) = router.find_route(
                    method.to_string(),
                    path.trim_left_matches(&routing).to_string(),
                )?;

                return Ok((method, params));
            }
        }

        return Err(Error::new(ErrorKind::NotFound, "404"));
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
                Err(e) => panic!(e), // TODO: Redirect to internal Router error page.
            };

            let self_clone = shared_self.clone();
            pool.execute(move || {
                self_clone.parse_incoming(&mut stream);
            });
        }
    }
}
