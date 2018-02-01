extern crate gtk;
extern crate glib;
extern crate gdk;

use gtk::*;
use gtk::prelude::*;
use glib::signal::SignalHandlerId;

use main_window::header::*;
use main_window::content::*;

pub struct MainWindow {
    pub window: gtk::Window,
    pub header: Header,
    pub contents: Content,
}

fn delete_event(s: &gtk::Window, f: &gdk::Event) -> Inhibit {
    gtk::main_quit();
    Inhibit(false)
}

impl MainWindow {
    pub fn new(builder: &Builder) -> MainWindow {
        let main_window = MainWindow {
            window: builder.get_object::<Window>("window1").unwrap(),
            header: Header::new(builder),
            contents: Content::new(builder),
        };

        main_window.window.set_title("Filer");
        main_window
    }
    pub fn init(&self) {
        self.window.connect_delete_event(&delete_event);
    }
    pub fn show(&self) {
        self.window.show_all();
    }
}
