//maybe take some inspiration from here
//https://gitlab.gnome.org/danigm/fractal/blob/master/fractal-gtk/src/widgets/member.rs

//very old tutorial for event box:
//https://www.cc.gatech.edu/data_files/public/doc/gtk/tutorial/gtk_tut-9.html

//very old tutorial for writing a widget from scratch *this is not needed!
// https://www.cc.gatech.edu/data_files/public/doc/gtk/tutorial/gtk_tut-20.html

extern crate gtk;

use gtk::*;
use gtk::prelude::*;

use std::vec;

// Apparently its the easiest thing to build or own widget,
// instead of relying on the gtks build-in ones for a good grid view.
// gtk::IconView is not very useful and implementing it myself is good
// exercise.
// The idea is to set everything up with boxes in the event box
// and use it outside in the container. The question are stuff like the scroll windows,
// but this should be easy in theory?

// icon grid containing icons with label below:
// +-------+
// | duck  |
// | duck  |
// | goose |
// +--------
// | quack |
// +-------+

pub struct IconGridView {
    pub row_count: u32,
    pub col_count: u32,
    pub widget: gtk::EventBox,
    pub rows: Vec<IconGridViewRow>,
}

pub struct IconGridViewRow {
    pub col_count: u32,
    //horizontal box
    pub box_widget: gtk::Box,
    pub cells: Vec<IconGridViewCell>,
}

pub struct IconGridViewCell {
    //vertical box
    pub box_widget: gtk::Box,
    pub icon: gtk::Image,
    pub label: gtk::Label,

    //if not sufficient use gtk::drawing area here and connect_draw event
}

