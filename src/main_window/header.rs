extern crate gtk;

use std::cell::Cell;
use std::rc::Rc;

use gtk::*;
use gtk::prelude::*;

use main_window::window::MainWindow;

#[derive(Clone)]
pub struct Header {
    pub header_bar: HeaderBar,
    pub back_button: Button,
    pub forward_button: Button,
    pub up_button: Button,
    pub path_entry: Entry,
    pub find_toggle_button: ToggleButton,
    pub details_view_toggle_button: ToggleButton,
    pub icons_view_toggle_button: ToggleButton,
    pub test_option: Rc<Cell<bool>>,
}

impl Header {
    pub fn new(builder: &Builder) -> Header {
        Header {
            header_bar: builder.get_object::<HeaderBar>("header").unwrap(),
            back_button: builder.get_object::<Button>("back_button").unwrap(),
            forward_button: builder.get_object::<Button>("forward_button").unwrap(),
            up_button: builder.get_object::<Button>("up_button").unwrap(),
            path_entry: builder.get_object::<Entry>("path_entry").unwrap(),
            find_toggle_button: builder
                .get_object::<ToggleButton>("find_toggle_button")
                .unwrap(),
            details_view_toggle_button: builder
                .get_object::<ToggleButton>("details_view_toggle_button")
                .unwrap(),
            icons_view_toggle_button: builder
                .get_object::<ToggleButton>("icons_view_toggle_button")
                .unwrap(),
            test_option: Rc::new(Cell::new(false)),
        }
    }

    pub fn is_any_view_toogle_button_active(&self) -> bool {
        self.details_view_toggle_button.get_active() || self.icons_view_toggle_button.get_active()
    }
}

impl Header {
    pub fn startup(&self, main_window: &MainWindow, _app: &gtk::Application) {
        let header = self.clone();
        self.icons_view_toggle_button
            .connect_clicked(move |button| {
                if !header.is_any_view_toogle_button_active() {
                    button.set_active(true);
                    println!("TODO: Show GtkIconView on center column");
                } else {
                    header.details_view_toggle_button.set_active(false);
                }
            });

        let header = self.clone();
        self.details_view_toggle_button
            .connect_clicked(move |button| {
                if !header.is_any_view_toogle_button_active() {
                    button.set_active(true);
                    println!("TODO: Show GtkTreeView on center column");
                } else {
                    header.icons_view_toggle_button.set_active(false);
                }
            });

        let search_bar = main_window.contents.search_bar.clone();
        self.find_toggle_button.connect_clicked(move |button| {
            search_bar.set_search_mode(button.get_active());
        });

        let test_option = self.test_option.clone();
        self.back_button.connect_clicked(move |_button| {
            let old_value = test_option.get();
            test_option.set(!old_value);
        });

        let test_option = self.test_option.clone();
        self.forward_button.connect_clicked(move |_button| {
            let value = test_option.get();
            println!("value of test_option was: {}", value);
        });
    }
}
