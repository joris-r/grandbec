extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

pub struct GuiGoods {
    data : Rc<RefCell<Data>>,
    grid : gtk::Grid,
}

impl GuiGoods {
    pub fn new(data : &Rc<RefCell<Data>>) -> GuiGoods {
        let mut gs = GuiGoods {
            data : data.clone(),
            grid : gtk::Grid::new(),
        };
        gs.setup();
        gs
    }
    
    pub fn get_main_widget(&self) -> gtk::Widget {
        self.grid.clone().upcast::<gtk::Widget>()
    }
    
    fn setup(&mut self){
        self.grid.set_row_spacing(5);
        self.grid.set_column_spacing(10);
        let mut line = -1;
        
        let i_name = gtk::Label::new("");
        i_name.set_markup("<b>Ingrédients</b>");
        i_name.set_halign(gtk::Align::Start);
        line += 1;
        self.grid.attach(&i_name, 0, line, 1, 1);
        
        let i_qval = gtk::Label::new("");
        i_qval.set_markup("<b>Par défaut</b>");
        i_qval.set_halign(gtk::Align::Start);
        self.grid.attach(&i_qval, 1, line, 1, 1);
        
        let i_qunit = gtk::Label::new("");
        i_qunit.set_markup("<b>Unité</b>");
        i_qunit.set_halign(gtk::Align::Start);
        self.grid.attach(&i_qunit, 2, line, 1, 1);
        
        let i_group_name = gtk::Label::new("");
        i_group_name.set_markup("<b>Groupe alimentaire</b>");
        i_group_name.set_halign(gtk::Align::Start);
        self.grid.attach(&i_group_name, 3, line, 1, 1);
        
        let i_section_name = gtk::Label::new("");
        i_section_name.set_markup("<b>Rayon de magasin</b>");
        i_section_name.set_halign(gtk::Align::Start);
        self.grid.attach(&i_section_name, 4, line, 1, 1);
        
        for i in self.data.borrow().iter_ingredients() {
        
            let i_name = gtk::Label::new(&i.name as &str);
            i_name.set_halign(gtk::Align::Start);
            line += 1;
            self.grid.attach(&i_name, 0, line, 1, 1);
            
            let i_qval = gtk::Label::new(&i.quantity.val.to_string() as &str);
            i_qval.set_halign(gtk::Align::Start);
            self.grid.attach(&i_qval, 1, line, 1, 1);
            
            let i_qunit = gtk::Label::new(&i.quantity.unit.to_string() as &str);
            i_qunit.set_halign(gtk::Align::Start);
            self.grid.attach(&i_qunit, 2, line, 1, 1);
            
            let i_group_name = gtk::Label::new(&i.group.to_string() as &str);
            i_group_name.set_halign(gtk::Align::Start);
            self.grid.attach(&i_group_name, 3, line, 1, 1);
            
            let i_section_name = gtk::Label::new(&i.section.to_string() as &str);
            i_section_name.set_halign(gtk::Align::Start);
            self.grid.attach(&i_section_name, 4, line, 1, 1);
        }

    
    }
    
}
