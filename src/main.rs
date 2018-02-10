// #![feature(trace_macros)]
// trace_macros!(true);

//#[macro_use]
extern crate gtk;
extern crate glib;
extern crate gio;

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

        window_ref_startup.borrow_mut().init_menu(&app);
    });

    let window_ref_activate = window_ref.clone();
    app.connect_activate(move |app|{
        println!("app activated");

        window_ref_activate.borrow_mut().show(app);
    });

    app.run(&std::env::args().collect::<Vec<String>>());
    gtk::main();
    println!("End");
}

// fn create_test_window(app: &gtk::Application) {
//     //test
//     //this test window doesnt have client-side-decorations enabled by default
//     let other_app_window = gtk::ApplicationWindow::new(app);
//     other_app_window.set_show_menubar(true);
//
//     //to get client side decorations it needs a header bar!!
//     //https://stackoverflow.com/questions/21079506/how-do-client-side-decorations-work-with-gnome-3-10-and-gtk-3
//     let header = gtk::HeaderBar::new();
//     //TODO: there are some missing options i bet, set some options on headerl
//     header.set_visible(true);
//     header.set_show_close_button(true);
//     other_app_window.set_titlebar(&header);
//     other_app_window.show();
// }

//let button: Button = main_builder.get_object("button1").unwrap();
//let dialog: MessageDialog = main_builder.get_object("messagedialog1").unwrap();

//{
//let dialog = dialog.clone();
//dialog.hide();
//}

//{
//let dialog = dialog.clone();
//button.connect_clicked(move |_| {
//dialog.run();
//dialog.hide();
//});
//}

//let main_window: Window = main_builder.get_object::<Window>("window1").unwrap();
// list store is a little like a record set
//let list_store: ListStore = main_builder.get_object("filer_liststore").unwrap();

//let iter = list_store.append();
//let column = 0;
//let value = gtk::Value::from("teapot");

//list_store.set_value(&iter, column, &value);

//let window = Window::new(WindowType::Toplevel);
//window.set_title("First GTK+ Program");
//window.set_default_size(350, 70);
//let button = Button::new_with_label("Click me!");
//window.add(&button);
//window.show_all();

//window.connect_delete_event(|_, _| {
//gtk::main_quit();
//Inhibit(false)
//});

//button.connect_clicked(|_| {
//println!("Clicked!");
//});
