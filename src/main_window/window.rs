extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use std::fs;
use std::path::PathBuf;

use gtk::*;
use gtk::prelude::*;
use glib::signal::SignalHandlerId;

use gio::prelude::*;
use gio::{ApplicationFlags, Menu, MenuExt};

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

//TODO: save window state
//https://wiki.gnome.org/HowDoI/SaveWindowState

// #[derive(Clone)]
pub struct MainWindow {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub contents: Content,
    pub path_entry: gtk::Entry,
    pub content_box: gtk::Box,
    pub search_entry: gtk::SearchEntry,
    pub search_bar: gtk::SearchBar,
    pub main_menu: Option<gtk::Menu>,
    pub left_tree_view: gtk::TreeView,
    pub left_scrolled_window: gtk::ScrolledWindow,
    pub middle_scrolled_window: gtk::ScrolledWindow,
    pub right_scrolled_window: gtk::ScrolledWindow,
}

impl MainWindow {
    pub fn new(builder: &Builder) -> MainWindow {
        let main_window = MainWindow {
            window: builder
                .get_object::<ApplicationWindow>("main_application_window")
                .unwrap(),
            header: Header::new(builder),
            contents: Content::new(builder),
            path_entry: builder.get_object::<gtk::Entry>("path_entry").unwrap(),
            search_entry: builder
                .get_object::<gtk::SearchEntry>("search_entry")
                .unwrap(),
            search_bar: builder.get_object::<gtk::SearchBar>("search_bar").unwrap(),
            content_box: builder.get_object::<gtk::Box>("content_box").unwrap(),
            main_menu: None,
            left_tree_view: builder
                .get_object::<gtk::TreeView>("left_tree_view")
                .unwrap(),
            left_scrolled_window: builder
                .get_object::<gtk::ScrolledWindow>("left_scrolled_window")
                .unwrap(),
            middle_scrolled_window: builder
                .get_object::<gtk::ScrolledWindow>("middle_scrolled_window")
                .unwrap(),
            right_scrolled_window: builder
                .get_object::<gtk::ScrolledWindow>("right_scrolled_window")
                .unwrap(),
        };

        main_window.window.set_title("Filer");
        main_window
    }

    pub fn init(&self, app: &gtk::Application) {
        self.window
            .connect_delete_event(clone!(app => move |_window, _event| {
            app.quit();
            gtk::Inhibit(false)
        }));

        let header = &self.header;
        header
            .icons_view_toggle_button
            .connect_clicked(clone!(header => move |button| {
            if !header.is_any_view_toogle_button_active() {
                button.set_active(true);
            } else {
                header.details_view_toggle_button.set_active(false);
            }
        }));
        header
            .details_view_toggle_button
            .connect_clicked(clone!(header => move |button| {
            if !header.is_any_view_toogle_button_active() {
                button.set_active(true);
            } else {
                header.icons_view_toggle_button.set_active(false);
            }
        }));

        let search_bar = &self.search_bar;
        header
            .find_toggle_button
            .connect_clicked(clone!(search_bar => move |button| {
            search_bar.set_search_mode(button.get_active());
        }));

        let search_entry = &self.search_entry;
        search_entry.connect_stop_search(clone!(header => move |_search_entry| {
            header.find_toggle_button.set_active(false);
        }));

        let mut fileliststore = FileListStore::new();
        fileliststore.fill_from_path(&PathBuf::from("/home/laragana"));
        println!("file count: {}", fileliststore.count);
        self.left_tree_view.set_model(&fileliststore.list_store);
    }

    pub fn startup(&mut self, app: &gtk::Application) {
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

    pub fn shutdown(&self, _app: &gtk::Application) {
        println!("");
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
        //window actions
        let help_overlay_action = gio::SimpleAction::new("show-help-overlay", None);
        self.window.add_action(&help_overlay_action);

        let window = &self.window;
        help_overlay_action.connect_activate(
            clone!(window => move |_help_overlay_action, _maybe_variant| {
            //gtk_application_window_set_help_overlay ()
            //gtk_application_window_get_help_overlay ()
            let help_window: Option<gtk::ShortcutsWindow> = window.get_help_overlay();
            if let Some(help_window) = help_window {
                help_window.show();
            } else {
                show_info_message_box(&window, "Help me!");
            }
        }),
        );

        //app actions
        let preferences_action = gio::SimpleAction::new("preferences", None);
        app.add_action(&preferences_action);

        let help_action = gio::SimpleAction::new("help", None);
        app.add_action(&help_action);
        help_action.connect_activate(clone!(window => move |_, _| {
            show_info_message_box(&window, "TODO: show overlay here");
            let result = show_yes_no_message_box(&window, "You won't, right?");
            if !result {
                println!("no you won't!");
            }
        }));

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
