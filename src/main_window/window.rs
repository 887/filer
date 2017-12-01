extern crate gtk;

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
    pub fn connect_delete_event(&self, f: & 'static Fn() -> Inhibit) {
        self.window.connect_delete_event(move |_, _| {
            f();
            Inhibit(false)
        });
    }
    pub fn show_all(&self) {
        self.window.show_all();
    }
}
