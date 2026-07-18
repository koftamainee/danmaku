use std::cell::RefCell;
use std::rc::Rc;

pub struct Scheduler {

}

pub type SharedScheduler = Rc<RefCell<Scheduler>>;

impl Scheduler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn shared() -> SharedScheduler {
        Rc::new(RefCell::new(Scheduler::new()))
    }
}