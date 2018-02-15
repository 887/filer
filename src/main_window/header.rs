use gtk::*;
use gtk::prelude::*;

#[derive(Clone)]
pub struct Header {
    pub header_bar: HeaderBar,
    pub back_button: Button,
    pub forward_button: Button,
    pub up_button: Button,
    pub path_entry: Entry,
    pub find_toggle_button: ToggleButton,
    pub details_view_toggle_button: ToggleButton,
    pub icons_view_toggle_button: ToggleButton,
}

impl Header {
    pub fn new(builder: &Builder) -> Header {
        let header = Header {
            header_bar: builder.get_object::<HeaderBar>("header").unwrap(),
            back_button: builder.get_object::<Button>("back_button").unwrap(),
            forward_button: builder.get_object::<Button>("forward_button").unwrap(),
            up_button: builder.get_object::<Button>("up_button").unwrap(),
            path_entry: builder.get_object::<Entry>("path_entry").unwrap(),
            find_toggle_button: builder
                .get_object::<ToggleButton>("find_toggle_button")
                .unwrap(),
            details_view_toggle_button: builder
                .get_object::<ToggleButton>("details_view_toggle_button")
                .unwrap(),
            icons_view_toggle_button: builder
                .get_object::<ToggleButton>("icons_view_toggle_button")
                .unwrap(),
        };

        header
    }

    pub fn is_any_view_toogle_button_active(&self) -> bool {
        self.details_view_toggle_button.get_active() ||
        self.icons_view_toggle_button.get_active()
    }
}
