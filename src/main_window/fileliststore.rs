use gtk::*;
use gtk::prelude::*;

use std::path::Path;

pub struct FileListStore {
    list_store: ListStore,
}

impl FileListStore {
    pub fn new() -> FileListStore {
        FileListStore {
            list_store: ListStore::new(&[Type::String, Type::String, Type::String]),
        }
    }
    pub fn fill_from_path(path: &Path){

    }
}


