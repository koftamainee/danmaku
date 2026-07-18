use std::cell::RefCell;
use std::rc::Rc;
use danmaku::{Danmaku, SharedDanmaku};

pub struct CoroutineManager {

}

pub type SharedCoroutineManager = Rc<RefCell<CoroutineManager>>;

impl CoroutineManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn shared() -> SharedCoroutineManager {
        Rc::new(RefCell::new(CoroutineManager::new()))
    }
}