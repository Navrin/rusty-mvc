use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

struct Payload {
    pub stringer: String,
}

pub trait Action: Send + Sync + 'static {
    fn call(&self, Payload) -> ();
}

impl<T> Action for T
where
    T: Fn(Payload) -> () + Send + Sync + 'static,
{
    fn call(&self, request: Payload) -> () {
        (&self)(request);
    }
}

struct ActionContainer {
    actions: Arc<RwLock<HashMap<String, Arc<Action>>>>,
}

impl ActionContainer {
    fn new() -> ActionContainer {
        ActionContainer {
            actions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn finder<T: ToString>(&self, query: T) -> Arc<Action> {
        let self_ref = self.actions.clone();
        let self_ref = self_ref.read().unwrap();
        let result = self_ref.get(&query.to_string());
        result.unwrap().clone()
    }
}


struct ActionDispatcher {
    actions_container: Arc<Mutex<ActionContainer>>,
}

impl ActionDispatcher {
    fn new() -> ActionDispatcher {
        ActionDispatcher {
            actions_container: Arc::new(Mutex::new(ActionContainer::new())),
        }
    }

    fn parser<T: ToString>(&self, chunk: T) -> Arc<Action> {
        let self_ref = self.actions_container.clone();
        let self_ref = self_ref.lock().unwrap();
        self_ref.finder(chunk)
    }

    fn caller<T: ToString>(&self, name: T) {
        let method = self.parser(name);
        method.call(Payload {
            stringer: "Hello!".to_string(),
        });
    }
}
