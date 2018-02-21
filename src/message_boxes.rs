extern crate gtk;

use gtk::*;
use glib::translate::*;

pub fn show_info_message_box(window: Option<&gtk::ApplicationWindow>, message: &str) {
    let mut flags = DialogFlags::USE_HEADER_BAR;
    if window != None {
        flags = flags | DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT;
    }
    let message_dialog = gtk::MessageDialog::new(
        window,
        flags,
        MessageType::Info,
        ButtonsType::Ok,
        message,
        );
    let _response = message_dialog.run();
    message_dialog.destroy();
}

pub fn show_yes_no_message_box(window: Option<&gtk::ApplicationWindow>, message: &str) -> bool {
    let mut flags = DialogFlags::USE_HEADER_BAR;
    if window != None {
        flags = flags | DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT;
    }
    let message_dialog = gtk::MessageDialog::new(
        window,
        flags,
        MessageType::Question,
        ButtonsType::YesNo,
        message,
        );
    let response = message_dialog.run();
    message_dialog.destroy();
    ResponseType::from_glib(response) == ResponseType::Yes
}
