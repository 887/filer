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
use glib::VariantType;
use glib::variant::FromVariant;

use gio::prelude::*;
use gio::{MenuExt, MenuItemExt, SettingsExt};

use consts::{FILER_SETTINGS_PREFERENCES, FILER_SETTINGS_WINDOW_STATE, FILER_WINDOW_INITIAL_HEIGHT,
             FILER_WINDOW_INITIAL_WIDTH, FILER_WINDOW_MAXIMIZED, FILER_WINDOW_SIDEBAR_WIDTH,
             FILER_WINDOW_START_WITH_SIDEBAR};

use message_boxes::{show_info_message_box,show_yes_no_message_box};

use filer_window::header::*;
use filer_window::content::*;

use widgets::icon_list_view::{IconListView, IconListViewColumn, IconListViewEntry};

use glib::translate::*;

// #[derive(Clone)]
pub struct FilerWindow {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub contents: Content,
    pub main_menu: Option<gtk::Menu>,
    pub settings_window_state: gio::Settings,
    pub settings_preferences: gio::Settings,
}

impl FilerWindow {
    pub fn new(builder: &Builder) -> FilerWindow {
        let filer_window = FilerWindow {
            window: builder
                .get_object::<ApplicationWindow>("main_application_window")
                .unwrap(),
            header: Header::new(builder),
            contents: Content::new(builder),
            main_menu: None,
            settings_preferences: gio::Settings::new(FILER_SETTINGS_PREFERENCES),
            settings_window_state: gio::Settings::new(FILER_SETTINGS_WINDOW_STATE),
        };

        filer_window.window.set_title("Filer");
        filer_window
    }

    pub fn startup(&mut self, app: &gtk::Application) {
        let maybe_menu = app.get_app_menu();
        if let Some(menu) = maybe_menu {
            // println!("Menu loaded from resources");
            self.main_menu = Some(gtk::Menu::new_from_model(&menu));
        } else {
            panic!("application resource loading is broken!");
        }

        self.map_window_actions();
        self.map_window_events(app);
        self.header.startup(self, app);
        self.contents.startup(self, app);
    }

    pub fn map_window_events(&self, app: &gtk::Application) {
        self.window
            .connect_delete_event(clone!(app => move |_window, _event| {
                let windows = app.get_windows().len();
                if windows <= 1 {
                    app.quit();
                }
                gtk::Inhibit(false)
            }));

        // https://wiki.gnome.org/HowDoI/SaveWindowState
        let settings_window_state = self.settings_window_state.clone();
        let paned = self.contents.navigation_paned.clone();
        self.window.connect_size_allocate(move |window, _event| {
            // save the window geometry only if we are not maximized or fullscreen
            if let Some(gdk_window) = window.get_window() {
                let window_state = gdk_window.get_state();
                if !(window_state == gdk::WindowState::FULLSCREEN) {
                    let (width, height) = window.get_size();
                    let maximized = window.is_maximized();
                    if !maximized {
                        settings_window_state.set_int(FILER_WINDOW_INITIAL_WIDTH, width);
                        settings_window_state.set_int(FILER_WINDOW_INITIAL_HEIGHT, height);
                    }
                    settings_window_state.set_boolean(FILER_WINDOW_MAXIMIZED, maximized);
                }
                let sidebar_width = paned.get_position();
                settings_window_state.set_int(FILER_WINDOW_SIDEBAR_WIDTH, sidebar_width);
            }
        });

        // self.window.connect_window_state_event(move |_window, _event| {
        //     gtk::Inhibit(false)
        // });
        //
        // self.window.connect_destroy(move |_window| {
        //     // _window.store_state();
        // });
    }

    fn map_window_actions(&mut self) {
        let window = &self.window;

        //window actions
        let settings_sidebar_visible = self.settings_window_state
            .get_boolean(FILER_WINDOW_START_WITH_SIDEBAR);
        let sidebar_action = gio::SimpleAction::new_stateful(
            "show-sidebar",
            None,
            &settings_sidebar_visible.to_variant(),
        );
        window.add_action(&sidebar_action);

        // gtk magic that should auto update the acion on change value. Doesn't seem to work though
        // self.settings_window_state.bind_writable(FILER_WINDOW_START_WITH_SIDEBAR, &sidebar_action, "visible", false);

        let places_sidebar = &self.contents.places_sidebar;

        let settings_window_state = &self.settings_window_state;
        sidebar_action.connect_activate(
            clone!(places_sidebar, settings_window_state => move |sidebar_action, _maybe_value| {
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
            clone!(window => move |_help_overlay_action, _maybe_value| {
                let help_overlay: Option<gtk::ShortcutsWindow> = window.get_help_overlay();
                if let Some(help_overlay) = help_overlay {
                    help_overlay.show();
                } else {
                    //TODO: load from resources
                    let ui_str = include_str!("../../data/gtk/shortcuts_filer_window.ui");
                    let builder: gtk::Builder = Builder::new_from_string(ui_str);
                    let new_help_overlay = builder.get_object::<gtk::ShortcutsWindow>("shortcuts_filer_window")
                        .unwrap();
                    window.set_help_overlay(&new_help_overlay);
                    new_help_overlay.show();
                }
            }),
        );
    }

    pub fn activate(&self, app: &gtk::Application) {
        app.add_window(&self.window);

        let initial_width = self.settings_window_state
            .get_int(FILER_WINDOW_INITIAL_WIDTH);
        let initial_height = self.settings_window_state
            .get_int(FILER_WINDOW_INITIAL_HEIGHT);
        let maximized = self.settings_window_state
            .get_boolean(FILER_WINDOW_MAXIMIZED);
        let sidebar_width = self.settings_window_state
            .get_int(FILER_WINDOW_SIDEBAR_WIDTH);

        self.window.set_default_size(initial_width, initial_height);
        if maximized {
            self.window.maximize();
        }

        self.contents.navigation_paned.set_position(sidebar_width);

        // let width = self.contents.postview_content_paned.get_position();
        self.contents.postview_content_paned.set_position(300);

        //show window first, then apply visible settings, otherwise it won't work
        self.window.show_all();

        //apply settings
        let settings_sidebar_visible = self.settings_window_state
            .get_boolean(FILER_WINDOW_START_WITH_SIDEBAR);
        if !settings_sidebar_visible {
            self.contents.places_sidebar.set_visible(false);
        }

        //acivate window contents and load them
        self.contents.activate(self);
    }

    pub fn shutdown(&self, _app: &gtk::Application) {
        // println!("");
    }
}
