#![allow(dead_code)]

use gtk::*;
use gtk::prelude::*;

// #[derive(Clone)]
pub struct Content {}

impl Content {
    pub fn new(_main_builder: &Builder) -> Content {
        let content = Content {};

        content
    }
}
