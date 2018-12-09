use std::cell::RefCell;
use std::rc::Rc;

mod store;
pub use self::store::*;

/// Used to instantiate our application
pub struct App {
    pub store: Rc<RefCell<Store>>,
}

impl App {
    /// Create a new instance of our WebGL Water application
    pub fn new() -> App {
        App {
            store: Rc::new(RefCell::new(Store::new())),
        }
    }
}

