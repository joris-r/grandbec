extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

pub struct GuiIngr {
    data : Rc<RefCell<Data>>
}

impl GuiIngr {
    pub fn new(data : &Rc<RefCell<Data>>) -> GuiIngr {
        let mut gs = GuiIngr {
            data : data.clone(),
        };
        gs.setup();
        gs
    }
    
    fn setup(&mut self){
        
        let grid = gtk::Grid::new();
        grid.set_orientation(gtk::Orientation::Vertical);
        
        grid.add(&gtk::Label::new("Ingrédients"));
        
        for i in recipe.ingredients.values() {
            let s = format!("{} {}", (*i).quantity, i.name);
            grid.add(&gtk::Label::new(&s as &str));
        }

    
    }
    
}
