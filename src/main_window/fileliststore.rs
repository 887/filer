use gtk::*;
use gtk::prelude::*;
use gtk::ListStoreExt;

use std::path::Path;
use std::fs;

pub struct FileListStore {
    list_store: ListStore,
}

impl FileListStore {
    pub fn new() -> FileListStore {
        FileListStore {
            list_store: ListStore::new(&[Type::String, Type::String, Type::String]),
        }
    }
    pub fn fill_from_path(&mut self, path: &Path){
        let paths = fs::read_dir(path);
        for path in paths {
            let tree_iter = self.list_store.append();
            let tree_iter = self.list_store.set(
                &tree_iter,
                &[0, 1, 2],
                &[&String::from(""), &String::from(""), &String::from("")]);
        }
    }
}


