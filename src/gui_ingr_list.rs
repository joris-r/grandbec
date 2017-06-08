extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

use gui_recipe::*;

pub struct GuiIngrList {
    data : Rc<RefCell<Data>>,
    list : gtk::ListBox,
}

impl GuiIngrList {
    pub fn new(data : &Rc<RefCell<Data>>) -> GuiIngrList {
        let mut gs = GuiIngrList {
            data : data.clone(),
            list : gtk::ListBox::new(),
        };
        gs.setup();
        gs
    }
    
    pub fn get_main_widget(&self) -> gtk::Widget {
        self.list.clone().upcast::<gtk::Widget>()
    }
    
    fn setup(&mut self){
        
        // workaround for the activation problem
        self.list.set_selection_mode(gtk::SelectionMode::Single);
        self.list.connect_row_selected(move |_,olbr| {
            match olbr {
                &Some(ref lbr) => {lbr.activate();},
                _ => {},
            }
        });
        
    }
    
    pub fn depopulate(&self) {
        for w in self.list.get_children() {
            self.list.remove(&w);
        }
    }
    
    pub fn populate_ingr(&self, target : &Rc<RefCell<GuiRecipe>>) {
        self.depopulate();
        
        let title = gtk::Label::new("");
        title.set_markup("<b>Ingrédients</b>");
        let row = gtk::ListBoxRow::new();
        row.add(&title);
        row.set_selectable(false);
        self.list.add(&row);
        
        for i in self.data.borrow().iter_ingredients() {
            let name = gtk::Label::new(&i.name as &str);
            let row = gtk::ListBoxRow::new();
            row.add(&name);
            self.list.add(&row);
            
            let target_clone = target.clone();
            let i_clone = i.clone();
            row.connect_activate(move |_| {
                target_clone.borrow_mut().add_ingredient(&i_clone);
            });

        }
        
        let title = gtk::Label::new("");
        let row = gtk::ListBoxRow::new();
        row.add(&title);
        row.set_selectable(false);
        self.list.add(&row);
        
        let title = gtk::Label::new("");
        title.set_markup("<b>Recettes</b>");
        let row = gtk::ListBoxRow::new();
        row.add(&title);
        row.set_selectable(false);
        self.list.add(&row);
        
        for i in self.data.borrow().iter_recipes() {
            let name = gtk::Label::new(&i.name as &str);
            let row = gtk::ListBoxRow::new();
            row.add(&name);
            self.list.add(&row);
        }
        
        self.list.show_all();
    }
    
}
