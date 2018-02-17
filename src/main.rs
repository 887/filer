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
use gio::Resource;

use main_window::window::MainWindow;

const APP_ID: &str = "a887.filer";

#[cfg(feature = "experiments")]
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

    //TODO: use meson to build and install everything in the appropriate paths.
    //example for that: https://gitlab.gnome.org/danigm/fractal/blob/master/
    //temporary development workaround for debug builds, do not include in release build
    {
        // GtkApplication will automatically load menus from the GtkBuilder resource located at "gtk/menus.ui",
        let resources_file = concat!(env!("CARGO_MANIFEST_DIR"), "/res/resources.gresource");
        println!("{}", &("RESOURCES:".to_string() + resources_file));
        let resource = gio::Resource::load(resources_file).unwrap();
        //https://developer.gnome.org/gio/unstable/GResource.html#g-resources-register
        gio::resources_register(&resource);

        // https://askubuntu.com/questions/251712/how-can-i-install-a-gsettings-schema-without-root-privileges
        //https://doc.rust-lang.org/1.15.0/std/env/
        // GSETTINGS_SCHEMA_DIR=~/schemas
        let resources_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/res/");
        std::env::set_var("GSETTINGS_SCHEMA_DIR", resources_dir);
    }

    let app: gtk::Application = app_result.unwrap();

    let main_glade = include_str!("../res/main_window.glade");
    let main_builder: Builder = Builder::new_from_string(main_glade);

    app.connect_startup(move |app| {
        use std::env;

        let mut main_window = MainWindow::new(&main_builder);
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
