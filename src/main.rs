// #![feature(trace_macros)]
// trace_macros!(true);

//#[macro_use]
extern crate gtk;
extern crate glib;

mod main_window;

use gtk::prelude::*;
use gtk::*;

use main_window::window::MainWindow;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let main_glade = include_str!("ui/filer.glade");
    let main_builder: Builder = Builder::new_from_string(main_glade);

    let main_window = MainWindow::new(&main_builder);

    main_window.init();

    main_window.show();

    gtk::main();
    println!("End");
}


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
