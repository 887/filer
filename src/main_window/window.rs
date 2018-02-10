extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate gio;

use gtk::*;
use gtk::prelude::*;
use glib::signal::SignalHandlerId;

use gio::prelude::*;
use gio::MenuExt;
use gio::ApplicationFlags;
use gio::Menu;

use main_window::header::*;
use main_window::content::*;
use main_window::fileliststore::*;

#[derive(Clone)]
pub struct MainWindow {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub contents: Content,
    pub main_menu: Option<gtk::Menu>,
}

fn delete_event(s: &gtk::ApplicationWindow, f: &gdk::Event) -> Inhibit {
    gtk::main_quit();
    Inhibit(false)
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
    pub fn init(&self) {
        self.window.connect_delete_event(&delete_event);

        let fileliststore = FileListStore::new();
    }
    pub fn init_menu(&mut self, app: &gtk::Application) {
        //https://wiki.gnome.org/HowDoI/ApplicationMenu
        //http://gtk-rs.org/docs/gio/struct.Menu.html

        let maybe_menu = app.get_app_menu();
        if let Some(menu) = maybe_menu  {
            self.connect_menu(menu);
        } else {
            self.create_menu(&app);
        }
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

        let quit_menu_item = gio::MenuItem::new("_Quit","app.quit");
        menu_actions.append_item(&quit_menu_item);

        menu_main.append_section(None, &menu_actions);

        //this is expected to be done during application statup, otherwise it wont work
        app.set_app_menu(&menu_main);

        self.main_menu = Some(gtk::Menu::new_from_model(&menu_main));
    }
    fn connect_menu(&mut self, menu_main: gio::MenuModel) {
        self.main_menu = Some(gtk::Menu::new_from_model(&menu_main));
    }
    pub fn show(&self, app: &gtk::Application) {
        //add window to appplication. This show the app menu when needed
        app.add_window(&self.window);
        self.window.show_all();
    }
}
