use gtk::*;
use gtk::prelude::*;

pub struct Header {
    pub header_bar: HeaderBar,
    pub options_button: Button
}

impl Header {
    pub fn new(main_builder: &Builder) -> Header {
        let header = Header {
            header_bar: main_builder.get_object::<HeaderBar>("header").unwrap(),
            options_button: main_builder.get_object::<Button>("options_button").unwrap(),
        };

        header
    }
}



