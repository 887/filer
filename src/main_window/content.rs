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
use main_window::window::MainWindow;

// #[derive(Clone)]
pub struct Content {
    pub content_box: gtk::Box,
    pub left_tree_view: gtk::TreeView,
    pub left_scrolled_window: gtk::ScrolledWindow,
    pub middle_scrolled_window: gtk::ScrolledWindow,
    pub right_scrolled_window: gtk::ScrolledWindow,
    pub places_sidebar: gtk::PlacesSidebar,
    pub search_entry: gtk::SearchEntry,
    pub search_bar: gtk::SearchBar,
}

impl Content {
    pub fn new(builder: &Builder) -> Content {
        let content = Content {
            content_box: builder.get_object::<gtk::Box>("content_box").unwrap(),
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
            search_entry: builder
                .get_object::<gtk::SearchEntry>("search_entry")
                .unwrap(),
            search_bar: builder.get_object::<gtk::SearchBar>("search_bar").unwrap(),
        };

        content
    }

    pub fn startup(&self, main_window: &MainWindow, _app: &gtk::Application) {
        let header = &main_window.header;
        self.search_entry
            .connect_stop_search(clone!(header => move |_search_entry| {
            header.find_toggle_button.set_active(false);
        }));
    }

    pub fn activate(&self, _window: &MainWindow) {
        let mut fileliststore = FileListStore::new();
        fileliststore.fill_from_path(&PathBuf::from("/home/laragana"));
        println!("file count: {}", fileliststore.count);
        self.left_tree_view.set_model(&fileliststore.list_store);

        // if window.header.... todo
    }
}
