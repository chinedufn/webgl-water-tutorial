use std::cell::RefCell;
use std::rc::Rc;

mod store;
pub use self::store::*;

mod assets;
pub use self::assets::*;

/// Used to instantiate our application
pub struct App {
    assets: Assets,
    pub store: Rc<RefCell<Store>>,
}

impl App {
    /// Create a new instance of our WebGL Water application
    pub fn new() -> App {
        let mut assets = Assets::new();

        App {
            assets,
            store: Rc::new(RefCell::new(Store::new())),
        }
    }

    pub fn assets(&self) -> &Assets {
        &self.assets
    }
}
