use std::cell::Cell;
use std::rc::Rc;

pub struct Preferences {
    pub on: Rc<Cell<bool>>,
}
