use server::request::Request;
use server::response::Response;
use std::sync::mpsc::Sender;

pub trait MiddlewareMethod: Send + Sync + 'static {
    fn call(&self, &Request, &mut Response, MiddlewareSession) -> ();
}

impl<T> MiddlewareMethod for T
where
    T: Fn(&Request, &mut Response, MiddlewareSession) -> () + Send + Sync + 'static,
{
    fn call(
        &self,
        request: &Request,
        mut response: &mut Response,
        session: MiddlewareSession,
    ) -> () {
        (&self)(request, response, session);
    }
}

impl MiddlewareMethod for Box<MiddlewareMethod> {
    fn call(
        &self,
        request: &Request,
        mut response: &mut Response,
        session: MiddlewareSession,
    ) -> () {
        (**self).call(request, response, session);
    }
}

impl Drop for MiddlewareSession {
    fn drop(&mut self) {
        println!("Stop drop and roll");
        self.terminate();
    }
}

pub struct MiddlewareSession {
    invoker: Sender<bool>,
}

impl MiddlewareSession {
    pub fn new(invoker: Sender<bool>) -> MiddlewareSession {
        MiddlewareSession { invoker }
    }

    pub fn next(&mut self) {
        self.invoker.send(true);
    }

    pub fn terminate(&mut self) {
        self.invoker.send(false);
    }
}
