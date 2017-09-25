use server::middleware::MiddlewareMethod;
use std::sync::{Arc, RwLock};

pub trait Sessionable: Send + Sync + 'static {
    fn parse(&self) -> Session;
}

impl<T: MiddlewareMethod> Sessionable for T {
    fn parse(&self) -> Session {
        let mut session = Session::new();
        session.then(*self);
        session
    }
}

impl Sessionable for Session {
    fn parse(&self) -> Session {
        *self
    }
}

impl Sessionable for &'static Session {
    fn parse(&self) -> Session {
        *self.clone()
    }
}

pub struct Session {
    pub before: Arc<RwLock<Vec<Box<MiddlewareMethod>>>>,
    pub then: Arc<RwLock<Vec<Box<MiddlewareMethod>>>>,
    pub after: Arc<RwLock<Vec<Box<MiddlewareMethod>>>>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            before: Arc::new(RwLock::new(Vec::new())),
            then: Arc::new(RwLock::new(Vec::new())),
            after: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn then<T: MiddlewareMethod>(&mut self, method: T) -> &mut Session {
        let wares_rw = self.then.clone();
        let mut wares = wares_rw.write().unwrap();
        wares.push(Box::new(method));
        self
    }

    pub fn before<T: MiddlewareMethod>(&mut self, method: T) -> &mut Session {
        let wares_rw = self.before.clone();
        let mut wares = wares_rw.write().unwrap();
        wares.push(Box::new(method));
        self
    }

    pub fn after<T: MiddlewareMethod>(&mut self, method: T) -> &mut Session {
        let wares_rw = self.after.clone();
        let mut wares = wares_rw.write().unwrap();
        wares.push(Box::new(method));
        self
    }
}
