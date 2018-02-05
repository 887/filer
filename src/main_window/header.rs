use gtk::*;
use gtk::prelude::*;

pub struct Header {
    pub header_bar: HeaderBar,
    pub file_menu_button: MenuButton,
    pub back_button: Button,
    pub forward_button: Button,
    pub up_button: Button,
    pub find_toggle_button: ToggleButton,
    pub details_view_toggle_button: ToggleButton,
    pub icons_view_toggle_button: ToggleButton,
    pub names_view_toggle_button: ToggleButton,
    pub main_stack_switcher: StackSwitcher,
}

impl Header {
    pub fn new(main_builder: &Builder) -> Header {
        let header = Header {
            header_bar: main_builder.get_object::<HeaderBar>("header").unwrap(),
            file_menu_button: main_builder.get_object::<MenuButton>("file_button").unwrap(),
            back_button: main_builder.get_object::<Button>("back_button").unwrap(),
            forward_button: main_builder.get_object::<Button>("forward_button").unwrap(),
            up_button: main_builder.get_object::<Button>("up_button").unwrap(),
            find_toggle_button: main_builder.get_object::<ToggleButton>("find_toggle_button").unwrap(),
            details_view_toggle_button: main_builder.get_object::<ToggleButton>("details_view_toggle_button").unwrap(),
            icons_view_toggle_button: main_builder.get_object::<ToggleButton>("icons_view_toggle_button").unwrap(),
            names_view_toggle_button: main_builder.get_object::<ToggleButton>("names_view_toggle_button").unwrap(),
            main_stack_switcher: main_builder.get_object::<StackSwitcher>("main_stack_switcher").unwrap(),
        };

        header.names_view_toggle_button.connect_clicked(|_| {
            println!("test");
        });

        header.names_view_toggle_button.emit_activate();

        header
    }
}



