#![allow(unused_imports)]
#![allow(dead_code)]

// #![feature(trace_macros)]
// trace_macros!(true);

//#[macro_use]
extern crate gio;
extern crate glib;
extern crate gtk;

mod main_window;

use std::rc::Rc;
use std::cell::RefCell;

use gtk::*;
use gio::prelude::*;
use gio::ApplicationFlags;

use main_window::window::MainWindow;

const APP_ID: &str = "a887.Filer";

#[cfg(feature = "experiments")]
mod experiments;

// cargo run --features "experiments"
#[cfg(feature = "experiments")]
fn main() {
    println!("experiments");
    experiments::run_experiments();
}

#[cfg(not(feature = "experiments"))]
fn main() {
    let app_result = gtk::Application::new(APP_ID, ApplicationFlags::FLAGS_NONE);

    if app_result.is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let app: gtk::Application = app_result.unwrap();

    let main_glade = include_str!("gtk/filer.glade");
    let main_builder: Builder = Builder::new_from_string(main_glade);

    app.connect_startup(move |app| {
        use std::env;
        // GtkApplication will automatically load menus from the GtkBuilder resource located at "gtk/menus.ui",
        // The idea is to load the menu from the xml resources, but it does not seem to work.
        //
        // app.set_resource_base_path(env!("PWD"));
        // let pwd = "PWD:".to_string() + env!("PWD");
        // println!("{}", &pwd);

        let mut main_window = MainWindow::new(&main_builder);
        main_window.init(&app);
        main_window.startup(app);

        let window_ref = Rc::new(RefCell::new(main_window));

        let window_ref_activate = window_ref.clone();
        app.connect_activate(move |app| {
            window_ref_activate.borrow_mut().activate(app);
        });

        let window_ref_shutdown = window_ref.clone();
        app.connect_shutdown(move |app| {
            window_ref_shutdown.borrow_mut().shutdown(app);
            println!("End");
        });
    });

    app.run(&std::env::args().collect::<Vec<String>>());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
