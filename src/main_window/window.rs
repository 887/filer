extern crate gtk;
extern crate glib;
extern crate gdk;

use gtk::*;
use gtk::prelude::*;
use glib::signal::SignalHandlerId;

use main_window::header::*;
use main_window::content::*;
use main_window::fileliststore::*;

pub struct MainWindow {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub contents: Content,
}

fn delete_event(s: &gtk::ApplicationWindow, f: &gdk::Event) -> Inhibit {
    gtk::main_quit();
    Inhibit(false)
}

impl MainWindow {
    pub fn new(builder: &Builder) -> MainWindow {
        let main_window = MainWindow {
            window: builder.get_object::<ApplicationWindow>("window1").unwrap(),
            header: Header::new(builder),
            contents: Content::new(builder),
        };

        main_window.window.set_title("Filer");
        main_window
    }
    pub fn init(&self) {
        self.window.connect_delete_event(&delete_event);

        let fileliststore = FileListStore::new();
    }
    pub fn show(&self, app: &gtk::Application) {
        //TODO: read the docs
        //can we set an app to this window this late?
        //it seems to be only possible in new..
        self.window.show_all();
    }
}
