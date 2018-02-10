#[allow(dead_code)]

use gtk::*;
use gio::prelude::*;
use gio::ApplicationFlags;

// file to experiment with gtk

pub fn run_experiments(app: &gtk::Application) {
    create_test_window(app);
}

fn create_test_window(app: &gtk::Application) {
    let other_app_window = gtk::ApplicationWindow::new(app);

    //this test window doesnt have client-side-decorations enabled by default
    other_app_window.set_show_menubar(true);

    //to get client side decorations it needs a header bar!!
    //https://stackoverflow.com/questions/21079506/how-do-client-side-decorations-work-with-gnome-3-10-and-gtk-3
    let header = gtk::HeaderBar::new();
    header.set_visible(true);
    header.set_show_close_button(true);

    other_app_window.set_titlebar(&header);
    other_app_window.show();
}

