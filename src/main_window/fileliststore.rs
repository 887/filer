#![allow(dead_code)]

extern crate gdk;
extern crate gtk;

use gtk::*;
use gtk::prelude::*;
use gtk::ListStoreExt;

use gtk::*;
use gtk::prelude::*;

use std::path::PathBuf;
use std::fs;

pub struct FileListStore {
    pub list_store: ListStore,
    pub count: i32,
}

impl FileListStore {
    pub fn new() -> FileListStore {
        FileListStore {
            list_store: ListStore::new(&[Type::String, Type::String, Type::String, Type::BaseObject]),
            count: 0,
        }
    }

    pub fn fill_from_path(&mut self, path: &PathBuf) {
        let paths = fs::read_dir(path).unwrap();
        let mut count = 0;
        let image = gtk::Image::new_from_icon_name("image-x-generic", gtk::IconSize::Menu.into());
        for _path in paths {
            let tree_iter = self.list_store.append();
            let _tree_iter = self.list_store.set(
                &tree_iter,
                &[0, 1, 2, 3],
                &[&String::from("a"), &String::from("b"), &String::from("image-x-generic"), &image.get_pixbuf()],
            );
            count += 1;
        }
        self.count = count;
    }
}
