extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use std::fs;
use std::path::PathBuf;
use std::env::home_dir;

use gtk::*;
use gtk::prelude::*;
use glib::signal::SignalHandlerId;
use glib::VariantType;
use glib::variant::FromVariant;

use gtk::*;
use gtk::prelude::*;

use filer_window::fileliststore::*;
use filer_window::window::FilerWindow;

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
    pub paned: gtk::Paned,
}

impl Content {
    pub fn new(builder: &Builder) -> Content {
        Content {
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
            paned: builder.get_object::<gtk::Paned>("paned").unwrap(),
        }
    }

    pub fn startup(&self, filer_window: &FilerWindow, _app: &gtk::Application) {
        let header = &filer_window.header;
        self.search_entry
            .connect_stop_search(clone!(header => move |_search_entry| {
            header.find_toggle_button.set_active(false);
        }));
    }

    pub fn activate(&self, _window: &FilerWindow) {
        let mut fileliststore = FileListStore::new();
        fileliststore.fill_from_path(&home_dir().unwrap());
        println!("file count: {}", fileliststore.count);
        self.left_tree_view.set_model(&fileliststore.list_store);

        let icon_view = self.create_icon_view(&fileliststore.list_store);
        self.middle_scrolled_window.add_with_viewport(&icon_view);

        self.middle_scrolled_window.connect_map_event(move |_middle_scrolled_window, _event| {
            // self.middle_scrolled_window.connect_size_allocate(move |_middle_scrolled_window, allocation| {
            Inhibit(false)
        });

        self.middle_scrolled_window.connect_size_allocate(move |_middle_scrolled_window, allocation| {
            // let item_width = icon_view.get_item_width();

            // let width = allocation.width;
            // if width > 0 {
            //     icon_view.set_item_width((width/4) -1);
            // }

            // println!("{}", width);

            // let scrollbar = _middle_scrolled_window.get_hscrollbar().unwrap();
            // if scrollbar.get_visible() {
            //     let _item_width = icon_view.set_item_width(width/2);
            // } else {
            //     let _item_width = icon_view.set_item_width(width/2 - 1);
            // }

            // let scrollbar = _middle_scrolled_window.get_hscrollbar().unwrap();
            // let columns = icon_view.get_columns();
            // if scrollbar.get_visible() {
            //         icon_view.set_columns(columns - 1);
            // middle_scrolled_window.set_width();
        });

        // let tree_view = self.create_tree_view(&fileliststore.list_store);
        // self.middle_scrolled_window.add_with_viewport(&tree_view);
    }

    // https://developer.gnome.org/gtk3/stable&fileliststore.list_store/GtkImage.html
    //
    // Creates a GtkImage displaying an icon f&fileliststore.list_storerom the current icon theme. If the icon name isn’t known, a “broken image” icon will be displayed instead. If the current icon theme is changed, the icon will be updated appropriately.
    // gtk_image_new_from_icon_name ()

    // https://developer.gnome.org/gtk3/stable/gtk3-Themeable-Stock-Images.html#GtkIconSize

    // https://wiki.gnome.org/HowDoI/DragAndDrop

    pub fn get_image() -> gtk::Image {
        gtk::Image::new_from_icon_name("image-x-generic", gtk::IconSize::Menu.into())
    }

    pub fn create_icon_view(&self, model: &ListStore) -> gtk::IconView {
        // let icon_view = IconView::new();
        // icon_view.set_model(model);

        // let icon_view = IconView::new_with_model(model);
        // icon_view.set_visible(true);
        // icon_view.set_columns(3);
        // icon_view.set_selection_mode(gtk::SelectionMode::Multiple);

        let ui_str = include_str!("../../data/gtk/file_icon_view.ui");
        let builder: gtk::Builder = Builder::new_from_string(ui_str);
        let icon_view = builder.get_object::<gtk::IconView>("file_icon_view").unwrap();
        icon_view.set_model(model);

        icon_view.set_text_column(0);
        icon_view.set_pixbuf_column(3);
        icon_view
    }

    pub fn create_tree_view(&self, model: &ListStore) -> gtk::TreeView {
        let ui_str = include_str!("../../data/gtk/file_tree_view.ui");
        let builder: gtk::Builder = Builder::new_from_string(ui_str);
        let tree_view = builder.get_object::<gtk::TreeView>("file_tree_view").unwrap();
        tree_view.set_model(model);
        // let tree_view = TreeView::new();
        // tree_view.set_model(model);
        // tree_view.set_visible(true);
        // //TODO:
        // //cant set cell renderer from gtk-rs, because its unimplemented,
        // //use builder .ui file for now
        // let column = TreeViewColumn::new();
        // tree_view.append_column(&column);
        tree_view
    }

    pub fn update_center_view() {}
}
