#![allow(unused_imports)]

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

const APP_ID: &str = "org.gnome.example";

fn main() {
    //https://github.com/gtk-rs/gtk/blob/master/src/auto/application.rs
    //https://developer.gnome.org/gtk3/stable/GtkApplication.html#gtk-application-new
    let app_res = gtk::Application::new(APP_ID, ApplicationFlags::FLAGS_NONE);

    if app_res.is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let app: gtk::Application = app_res.unwrap();

    let main_glade = include_str!("gtk/filer.glade");
    let main_builder: Builder = Builder::new_from_string(main_glade);

    let main_window = MainWindow::new(&main_builder);
    main_window.init();

    let window_ref = Rc::new(RefCell::new(main_window));

    let window_ref_startup = window_ref.clone();
    app.connect_startup(move |app| {
        use std::env;
        // GtkApplication will automatically load menus from the GtkBuilder resource located at "gtk/menus.ui",
        // this is the alternaive, but it does not seem to work.
        //
        // app.set_resource_base_path(env!("PWD"));
        // let pwd = "PWD:".to_string() + env!("PWD");
        // println!("{}", &pwd);

        window_ref_startup.borrow_mut().startup(&app);
    });

    let window_ref_activate = window_ref.clone();
    app.connect_activate(move |app| {
        println!("app activated");

        window_ref_activate.borrow_mut().activate(app);
    });

    app.run(&std::env::args().collect::<Vec<String>>());
    gtk::main();
    println!("End");
}
