#![allow(dead_code)]
#![allow(unused_variables)]

extern crate gio;
extern crate glib;
extern crate gtk;

use std;

use std::rc::Rc;
use std::cell::RefCell;

use gtk::*;
use gtk::prelude::*;
use glib::signal::SignalHandlerId;

use gio::prelude::*;
use gio::{ApplicationFlags, Menu, MenuExt};

use glib::translate::*;

// file to experiment with gtk

// this seems to be a real gnome app made in rust, that can be used as example?:
// https://gitlab.gnome.org/danigm/fractal

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move || $body
        }
        );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
        }
        );
}

pub fn run_experiments() {
    //https://github.com/gtk-rs/gtk/blob/master/src/auto/application.rs
    //https://developer.gnome.org/gtk3/stable/GtkApplication.html#gtk-application-new
    let app_res = gtk::Application::new(None, ApplicationFlags::FLAGS_NONE);

    if app_res.is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let app: gtk::Application = app_res.unwrap();

    app.connect_startup(move |app| {
        println!("App Startup");
        create_menu(app);

        let window = create_test_window(&app);
        app.connect_activate(clone!(window => move |app| {
            println!("App Activated");
            map_actions(app, &window);
            window.show();
        }));

        window.connect_delete_event(clone!(app => move |_, _| {
            println!("Window Deleted");
            app.quit();
            gtk::Inhibit(false)
        }));
    });

    app.connect_shutdown(move |_app| {
        println!("App Shutdown");
    });

    app.run(&std::env::args().collect::<Vec<String>>());
}

fn create_test_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::new(app);

    //this test window doesnt have client-side-decorations enabled by default
    window.set_show_menubar(true);

    //to get client side decorations it needs a header bar!!
    //https://stackoverflow.com/questions/21079506/how-do-client-side-decorations-work-with-gnome-3-10-and-gtk-3
    let header = gtk::HeaderBar::new();
    header.set_visible(true);
    header.set_show_close_button(true);

    window.set_titlebar(&header);
    window
}

fn show_info_message_box(window: &gtk::ApplicationWindow, message: &str) {
    let message_dialog = gtk::MessageDialog::new(
        Some(window),
        DialogFlags::MODAL | DialogFlags::USE_HEADER_BAR | DialogFlags::DESTROY_WITH_PARENT,
        MessageType::Info,
        ButtonsType::Ok,
        message,
    );
    let response = message_dialog.run();
    message_dialog.destroy();
    if ResponseType::from_glib(response) == ResponseType::Ok {
        println!("Ok clicked!");
    }
}

/// this is expected to be done during application statup, otherwise it wont work
fn create_menu(app: &gtk::Application) {
    //https://wiki.gnome.org/HowDoI/ApplicationMenu
    //http://gtk-rs.org/docs/gio/struct.Menu.html

    //here is a good example:
    //https://github.com/gtk-rs/examples/blob/master/src/bin/menu_bar_system.rs

    let menu_main = gio::Menu::new();

    menu_main.append("_Checkbox", "win.toggle-check-box");

    menu_main.append("_Radio Option", "win.toggle-radio(true)");
    menu_main.append("_Radio Option", "win.toggle-radio(false)");

    menu_main.append("_Keyboard Shortcuts", "win.show-help-overlay");
    menu_main.append("_About", "app.about");
    menu_main.append("_Quit", "app.quit");

    app.set_app_menu(&menu_main);
}

fn map_actions(app: &gtk::Application, window: &gtk::ApplicationWindow) {
    //https://wiki.gnome.org/HowDoI/GAction

    //window actions
    let help_action = gio::SimpleAction::new("show-help-overlay", None);
    window.add_action(&help_action);
    help_action.connect_activate(clone!(window => move |_, _| {
        //gtk_application_window_set_help_overlay ()
        //gtk_application_window_get_help_overlay ()
        let help_window: Option<gtk::ShortcutsWindow> = window.get_help_overlay();
        if let Some(help_window) = help_window {
            help_window.show();
        } else {
            show_info_message_box(&window, "Show Help Overlay here!");
        }
    }));

    // adding none, true here lets the action act as a check box with the checked option on
    let checkbox_action = gio::SimpleAction::new_stateful("toggle-check-box", None, &true.to_variant());
    window.add_action(&checkbox_action);

    //and this way its a radio button
    let ty = glib::VariantTy::new("b").unwrap();
    let radio_action = gio::SimpleAction::new_stateful("toggle-radio", ty, &true.to_variant());
    window.add_action(&radio_action);

    //app actions
    let preferences_action = gio::SimpleAction::new("about", None);
    app.add_action(&preferences_action);
    preferences_action.connect_activate(clone!(window => move |_, _| {
        show_info_message_box(&window, "You know to much about me!");
    }));
    let quit_action = gio::SimpleAction::new("quit", None);
    app.add_action(&quit_action);
    quit_action.connect_activate(clone!(window => move |_, _| {
        window.close();
    }));
}
