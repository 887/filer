//maybe take some inspiration from here
//https://gitlab.gnome.org/danigm/fractal/blob/master/fractal-gtk/src/widgets/member.rs

extern crate gtk;

use std::vec;

use gtk::*;
use gtk::prelude::*;

pub struct IconListView {
    pub widget: gtk::EventBox,
    pub columns: Vec<IconListViewColumn>,
    rows: u32,
    cols: u32,
}

pub struct IconListViewColumn {
    pub entries: Vec<gtk::Box>,
}

pub struct IconListViewEntry {
    pub icon: gtk::Image,
    pub label: gtk::Label,
}
