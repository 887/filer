extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use std::cell::Cell;
use std::rc::Rc;

use std::fs;
use std::path::PathBuf;

use gtk::*;
use gtk::prelude::*;

use self::gdk::WindowExt;

use glib::signal::SignalHandlerId;
use glib::{VariantType};
use glib::variant::FromVariant;

use gio::prelude::*;
use gio::{MenuExt, MenuItemExt, SettingsExt};

use main_window::header::*;
use main_window::content::*;

use glib::translate::*;

const FILER_SETTINGS_PREFERENCES: &str = "a887.filer.preferences";
const FILER_SETTINGS_WINDOW_STATE: &str = "a887.filer.window-state";
const FILER_WINDOW_START_WITH_SIDEBAR: &str = "start-with-sidebar";

// #[derive(Clone)]
pub struct MainWindow {
    pub window: gtk::ApplicationWindow,
    pub fullscreen: Rc<Cell<bool>>,
    pub header: Header,
    pub contents: Content,
    pub search_entry: gtk::SearchEntry,
    pub search_bar: gtk::SearchBar,
    pub main_menu: Option<gtk::Menu>,
    pub settings_window_state: gio::Settings,
    pub settings_preferences: gio::Settings,
}

impl MainWindow {
    pub fn new(builder: &Builder) -> MainWindow {
        let main_window = MainWindow {
            window: builder
                .get_object::<ApplicationWindow>("main_application_window")
                .unwrap(),
            fullscreen: Rc::new(Cell::new(false)),
            header: Header::new(builder),
            contents: Content::new(builder),
            search_entry: builder
                .get_object::<gtk::SearchEntry>("search_entry")
                .unwrap(),
            search_bar: builder.get_object::<gtk::SearchBar>("search_bar").unwrap(),
            main_menu: None,
            settings_preferences: gio::Settings::new(FILER_SETTINGS_PREFERENCES),
            settings_window_state: gio::Settings::new(FILER_SETTINGS_WINDOW_STATE),
        };

        main_window.window.set_title("Filer");
        main_window
    }

    pub fn startup(&mut self, app: &gtk::Application) {
        let maybe_menu = app.get_app_menu();
        if let Some(menu) = maybe_menu {
            println!("Menu loaded from resources");
            self.main_menu = Some(gtk::Menu::new_from_model(&menu));
        } else {
            //fallback if application resource loading is broken
            self.create_menu(&app);
        }

        self.map_app_actions(&app);
        self.map_window_actions();
        self.map_window_events(app);
        self.header.startup(&self, app);
    }

    fn create_menu(&mut self, app: &gtk::Application) {
        let menu_main = gio::Menu::new();

        let menu_sidebar = gio::Menu::new();
        // https://people.gnome.org/~gcampagna/docs/Gio-2.0/Gio.MenuModel.html

        let sidebar_menu_item = gio::MenuItem::new("_Show Sidebar", "win.show-sidebar");
        menu_sidebar.append_item(&sidebar_menu_item);
        // sidebar_menu_item.set_action_and_target_value("win.show-sidebar", &true.to_variant());
        menu_main.append_section(None, &menu_sidebar);

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

    fn map_app_actions(&mut self, app: &gtk::Application) {

        //app actions
        let preferences_action = gio::SimpleAction::new("preferences", None);
        app.add_action(&preferences_action);

        let help_action = gio::SimpleAction::new("help", None);
        app.add_action(&help_action);
        help_action.connect_activate(move |_, _| {
            show_info_message_box(None, "TODO: show something helpful here");
            let result = show_yes_no_message_box(None, "You won't, right?");
            if !result {
                println!("no you won't!");
            }
        });

        let about_action = gio::SimpleAction::new("about", None);
        app.add_action(&about_action);

        let quit_action = gio::SimpleAction::new("quit", None);
        app.add_action(&quit_action);

        let window = &self.window;
        quit_action.connect_activate(clone!(window => move |_, _| {
            window.close();
        }));
    }

    pub fn map_window_events(&self, app: &gtk::Application) {
        self.window
            .connect_delete_event(clone!(app => move |_window, _event| {
                app.quit();
                gtk::Inhibit(false)
            }));

        //https://wiki.gnome.org/HowDoI/SaveWindowState
        //TODO: save window
        let fullscreen = self.fullscreen.clone();
        self.window
            .connect_size_allocate(move |window, _event| {
                // save the window geometry only if we are not maximized of fullscreen
                // if !(self.window.is_maximized() || self.window.fullscreen()) {
                // You can track the fullscreen state via the “window-state-event”
                // signal on GtkWidget.

                //
                // TODO TRY THIS INSTEAD OF CELLLL
                // TODO: USE RC CELL TO SEE IF IT WORKS THAT WAY FOR LATER111 !!!
                let window_state = window.get_window().unwrap().get_state();

                if !(window.is_maximized() || window_state == gdk::WindowState::FULLSCREEN) {

                }
            }) ;

            let fullscreen = self.fullscreen.clone();
            self.window
                .connect_window_state_event(move |_window, event| {
                    let fullscreen = fullscreen.clone();

                    let window_state = event.get_new_window_state();
                    fullscreen.set(window_state == gdk::WindowState::FULLSCREEN);
                    println!("fullscreen: {}", window_state == gdk::WindowState::FULLSCREEN);
                    gtk::Inhibit(false)
                }) ;

            self.window.connect_destroy(move |_window| {
                // _window.store_state();
            });

    }

    fn map_window_actions(&mut self) {
        let window = &self.window;

        //window actions
        let settings_sidebar_visible = self.settings_window_state.get_boolean(FILER_WINDOW_START_WITH_SIDEBAR);
        let sidebar_action = gio::SimpleAction::new_stateful("show-sidebar", None, &settings_sidebar_visible.to_variant());
        window.add_action(&sidebar_action);

        // gtk magic that should auto update the acion on change value. Doesn't seem to work though
        // self.settings_window_state.bind_writable(FILER_WINDOW_START_WITH_SIDEBAR, &sidebar_action, "visible", false);

        let places_sidebar = &self.contents.places_sidebar;

        let settings_window_state = &self.settings_window_state;
        sidebar_action.connect_activate(
            clone!(places_sidebar, settings_window_state => move |sidebar_action, _maybe_variant_value| {
                let var_value = sidebar_action.get_state().unwrap();
                let sidebar_visible = !(bool::from_variant(&var_value).unwrap());
                places_sidebar.set_visible(sidebar_visible);
                settings_window_state.set_boolean(FILER_WINDOW_START_WITH_SIDEBAR, sidebar_visible);
                sidebar_action.set_state(&sidebar_visible.to_variant());
            }),
            );

        let help_overlay_action = gio::SimpleAction::new("show-help-overlay", None);
        window.add_action(&help_overlay_action);

        help_overlay_action.connect_activate(
            clone!(window => move |_help_overlay_action, _maybe_variant| {
                //gtk_application_window_set_help_overlay ()
                //gtk_application_window_get_help_overlay ()
                let help_window: Option<gtk::ShortcutsWindow> = window.get_help_overlay();
                if let Some(help_window) = help_window {
                    help_window.show();
                } else {
                    show_info_message_box(Some(&window), "Help me!");
                }
            }),
        );
    }

    pub fn activate(&self, app: &gtk::Application) {
        //add window to appplication. This show the app menu when needed
        app.add_window(&self.window);

        //show window first, then apply settings, otherwise it won't work
        self.window.show_all();

        //apply settings
        let settings_sidebar_visible = self.settings_window_state.get_boolean(FILER_WINDOW_START_WITH_SIDEBAR);
        if !settings_sidebar_visible { self.contents.places_sidebar.set_visible(false); }

        //acivate window contents and load them
        self.contents.activate(&self);
    }

    pub fn shutdown(&self, _app: &gtk::Application) {
        println!("");
    }

}

fn show_info_message_box(window: Option<&gtk::ApplicationWindow>, message: &str) {
    let message_dialog = gtk::MessageDialog::new(
        window,
        DialogFlags::MODAL | DialogFlags::USE_HEADER_BAR | DialogFlags::DESTROY_WITH_PARENT,
        MessageType::Info,
        ButtonsType::Ok,
        message,
    );
    let _response = message_dialog.run();
    message_dialog.destroy();
}

fn show_yes_no_message_box(window: Option<&gtk::ApplicationWindow>, message: &str) -> bool {
    let message_dialog = gtk::MessageDialog::new(
        window,
        DialogFlags::MODAL | DialogFlags::USE_HEADER_BAR | DialogFlags::DESTROY_WITH_PARENT,
        MessageType::Question,
        ButtonsType::YesNo,
        message,
    );
    let response = message_dialog.run();
    message_dialog.destroy();
    ResponseType::from_glib(response) == ResponseType::Yes
}
