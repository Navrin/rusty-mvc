use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, Box<(Fn() -> ())>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }
}
