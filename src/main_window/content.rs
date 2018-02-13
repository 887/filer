extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use std::fs;
use std::path::PathBuf;

use gtk::*;
use gtk::prelude::*;
use glib::signal::SignalHandlerId;
use glib::VariantType;
use glib::variant::FromVariant;

use gtk::*;
use gtk::prelude::*;

use main_window::fileliststore::*;

// #[derive(Clone)]
pub struct Content {
    pub left_tree_view: gtk::TreeView,
    pub left_scrolled_window: gtk::ScrolledWindow,
    pub middle_scrolled_window: gtk::ScrolledWindow,
    pub right_scrolled_window: gtk::ScrolledWindow,
    pub places_sidebar: gtk::PlacesSidebar,
}

impl Content {
    pub fn new(builder: &Builder) -> Content {
        let content = Content {
            left_tree_view: builder
                .get_object::<gtk::TreeView>("left_tree_view")
                .unwrap(),
            left_scrolled_window: builder
                .get_object::<gtk::ScrolledWindow>("left_scrolled_window")
                .unwrap(),
            middle_scrolled_window: builder
                .get_object::<gtk::ScrolledWindow>("middle_scrolled_window")
                .unwrap(),
            right_scrolled_window: builder
                .get_object::<gtk::ScrolledWindow>("right_scrolled_window")
                .unwrap(),
            places_sidebar: builder
                .get_object::<gtk::PlacesSidebar>("places_sidebar")
                .unwrap(),
        };

        content
    }
    pub fn activate(&self) {
        let mut fileliststore = FileListStore::new();
        fileliststore.fill_from_path(&PathBuf::from("/home/laragana"));
        println!("file count: {}", fileliststore.count);
        self.left_tree_view.set_model(&fileliststore.list_store);
    }
}
