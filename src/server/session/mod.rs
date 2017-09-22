use server::middleware::MiddlewareMethod;
use std::sync::{Arc, RwLock};

pub trait Sessionable<T: MiddlewareMethod>: Send + Sync + 'static {
    fn parse(self) -> Session<T>;
}

impl<T: MiddlewareMethod> Sessionable<T> for T {
    fn parse(self) -> Session<T> {
        let mut session = Session::new();
        session.then(self);
        session
    }
}

impl<T: MiddlewareMethod> Sessionable<T> for Session<T> {
    fn parse(self) -> Session<T> {
        self
    }
}

pub struct Session<T: MiddlewareMethod> {
    pub before: Arc<RwLock<Vec<Box<T>>>>,
    pub then: Arc<RwLock<Vec<Box<T>>>>,
    pub after: Arc<RwLock<Vec<Box<T>>>>,
}

impl<T: MiddlewareMethod> Session<T> {
    pub fn new() -> Session<T> {
        Session {
            before: Arc::new(RwLock::new(Vec::new())),
            then: Arc::new(RwLock::new(Vec::new())),
            after: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn then(&mut self, method: T) -> &mut Session<T> {
        let wares_rw = self.then.clone();
        let mut wares = wares_rw.write().unwrap();
        wares.push(Box::new(method));
        self
    }
}
