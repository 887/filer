extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use gtk::*;
use gtk::prelude::*;
use glib::signal::SignalHandlerId;

use gio::prelude::*;
use gio::{MenuExt, ApplicationFlags, Menu};

use main_window::header::*;
use main_window::content::*;
use main_window::fileliststore::*;

use glib::translate::*;

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

// #[derive(Clone)]
pub struct MainWindow {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub contents: Content,
    pub main_menu: Option<gtk::Menu>,
}

impl MainWindow {
    pub fn new(builder: &Builder) -> MainWindow {
        let main_window = MainWindow {
            window: builder.get_object::<ApplicationWindow>("window1").unwrap(),
            header: Header::new(builder),
            contents: Content::new(builder),
            main_menu: None,
        };

        main_window.window.set_title("Filer");
        main_window
    }

    pub fn init(&self, app: &gtk::Application) {
        self.window.connect_delete_event(clone!(app => move |_, _| {
            app.quit();
            gtk::Inhibit(false)
        }));
        let _fileliststore = FileListStore::new();
        //TODO
    }

    pub fn startup(&mut self, app: &gtk::Application) {
        //https://wiki.gnome.org/HowDoI/ApplicationMenu
        //http://gtk-rs.org/docs/gio/struct.Menu.html

        let maybe_menu = app.get_app_menu();
        if let Some(menu) = maybe_menu {
            self.main_menu = Some(gtk::Menu::new_from_model(&menu));
        } else {
            //fallback while application xml resource loading is broken
            self.create_menu(&app);
        }

        self.map_actions(&app);
    }

    pub fn activate(&self, app: &gtk::Application) {
        //add window to appplication. This show the app menu when needed
        app.add_window(&self.window);
        self.window.show_all();
    }

    fn create_menu(&mut self, app: &gtk::Application) {
        let menu_main = gio::Menu::new();

        let menu_preferences = gio::Menu::new();
        menu_preferences.append("Prefere_nces", "app.preferences");
        menu_main.append_section(None, &menu_preferences);

        let menu_actions = gio::Menu::new();
        menu_actions.append("_Keyboard Shortcuts", "win.show-help-overlay");
        menu_actions.append("_Help", "app.help");
        menu_actions.append("_About", "app.about");

        let quit_menu_item = gio::MenuItem::new("_Quit", "app.quit");
        menu_actions.append_item(&quit_menu_item);

        menu_main.append_section(None, &menu_actions);

        //this is expected to be done during application statup, otherwise it wont work
        app.set_app_menu(&menu_main);

        self.main_menu = Some(gtk::Menu::new_from_model(&menu_main));
    }

    fn map_actions(&mut self, app: &gtk::Application) {
        //https://wiki.gnome.org/HowDoI/GAction
        //here is a good example:
        //https://github.com/gtk-rs/examples/blob/master/src/bin/menu_bar_system.rs

        let help_overlay_action = gio::SimpleAction::new("show-help-overlay", None);
        self.window.add_action(&help_overlay_action);

        let window = &self.window;
        help_overlay_action.connect_activate(clone!(window => move |_, _| {
            show_info_message_box(&window, "TODO: show overlay here");
            let result = show_yes_no_message_box(&window, "You won't, right?");
            if !result {
                println!("no you won't!");
            }
        }));

        let preferences_action = gio::SimpleAction::new("preferences", None);
        app.add_action(&preferences_action);

        let help_action = gio::SimpleAction::new("help", None);
        app.add_action(&help_action);

        let about_action = gio::SimpleAction::new("about", None);
        app.add_action(&about_action);

        let quit_action = gio::SimpleAction::new("quit", None);
        app.add_action(&quit_action);
        quit_action.connect_activate(clone!(window => move |_, _| {
            window.close();
        }));

    }
}

fn show_info_message_box(window: &gtk::ApplicationWindow, message: &str) {
    let message_dialog = gtk::MessageDialog::new(
        Some(window),
        DialogFlags::MODAL | DialogFlags::USE_HEADER_BAR | DialogFlags::DESTROY_WITH_PARENT,
        MessageType::Info,
        ButtonsType::Ok,
        message,
    );
    let _response = message_dialog.run();
    message_dialog.destroy();
}

fn show_yes_no_message_box(window: &gtk::ApplicationWindow, message: &str) -> bool {
    let message_dialog = gtk::MessageDialog::new(
        Some(window),
        DialogFlags::MODAL | DialogFlags::USE_HEADER_BAR | DialogFlags::DESTROY_WITH_PARENT,
        MessageType::Question,
        ButtonsType::YesNo,
        message,
    );
    let response = message_dialog.run();
    message_dialog.destroy();
    ResponseType::from_glib(response) == ResponseType::Yes
}
