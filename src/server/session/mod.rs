use server::middleware::MiddlewareMethod;
use std::sync::Arc;

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

pub struct Session<T: MiddlewareMethod> {
    pub before: Arc<Vec<Box<T>>>,
    pub then: Arc<Vec<Box<T>>>,
    pub after: Arc<Vec<Box<T>>>,
}

impl<T: MiddlewareMethod> Session<T> {
    pub fn new() -> Session<T> {
        Session {
            before: Arc::new(Vec::new()),
            then: Arc::new(Vec::new()),
            after: Arc::new(Vec::new()),
        }
    }

    pub fn then(&mut self, method: T) -> &mut Session<T> {
        let mut ware_clone = self.then.clone();
        ware_clone.push(Box::new(method));
        self
    }
}
