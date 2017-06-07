extern crate gtk;
use gtk::prelude::*;

use logic::*;

pub fn create_combo_of_unit(unit : & Unit) -> gtk::ComboBoxText {
    let c = gtk::ComboBoxText::new();
    c.insert_text(Unit::Portion as i32, &Unit::Portion.to_string());
    c.insert_text(Unit::Gram as i32, &Unit::Gram.to_string());
    c.insert_text(Unit::Centilitre as i32, &Unit::Centilitre.to_string());
    c.set_active(*unit as i32);
    return c;
}

pub fn read_unit(widget : &gtk::ComboBoxText) -> Unit {
    let val = widget.get_active();
    if val == Unit::Portion as i32 { Unit::Portion }
    else if val == Unit::Gram as i32 { Unit::Gram }
    else if val == Unit::Centilitre as i32 { Unit::Centilitre }
    else { panic!("wrong Unit id") }
}
