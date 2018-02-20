#![allow(dead_code)]

extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;

use gtk::*;
use gtk::prelude::*;
use gtk::ListStoreExt;

use gtk::*;
use gtk::prelude::*;

use std::path::PathBuf;
use std::fs;

use std::ffi::OsString;

pub struct FileListStore {
    pub list_store: ListStore,
    pub count: i32,
}
// static_type()
impl FileListStore {
    pub fn new() -> FileListStore {
        FileListStore {
            list_store: ListStore::new(&[
                Type::String,
                Type::String,
                Type::String,
                gdk_pixbuf::Pixbuf::static_type(),
            ]),
            count: 0,
        }
    }

    pub fn fill_from_path(&mut self, path: &PathBuf) {
        let paths = fs::read_dir(path).unwrap();
        let mut count = 0;
        let image = gtk::Image::new_from_icon_name("folder", gtk::IconSize::Dialog.into());
        for path in paths {
            let tree_iter = self.list_store.append();
            let de: fs::DirEntry = path.unwrap();
            let file_name: OsString = de.file_name();
            let file_name_string = file_name.into_string().unwrap();
            let _tree_iter = self.list_store.set(
                &tree_iter,
                &[0, 1, 2, 3],
                &[
                    &String::from(file_name_string),
                    &String::from("b"),
                    &String::from("image-x-generic"),
                    &image.get_pixbuf(),
                ],
            );
            count += 1;
        }
        self.count = count;
    }
}
