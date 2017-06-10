extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

pub struct GuiPlanning {
    data : Rc<RefCell<Data>>,
    main_notebook : gtk::Notebook,
}

impl GuiPlanning {
    pub fn new(data : &Rc<RefCell<Data>>, main_notebook : &gtk::Notebook) -> GuiPlanning {
        let gs = GuiPlanning {
            data : data.clone(),
            main_notebook : main_notebook.clone(),
        };
        gs
    }
    
    fn setup(&mut self){
        
        let list = gtk::ListBox::new();
    
        // workaround for the activation problem
        list.set_selection_mode(gtk::SelectionMode::Single);
        list.connect_row_selected(move |_,olbr| {
            match olbr {
                &Some(ref lbr) => {lbr.activate();},
                _ => {},
            }
        });
        
        for section in self.data.borrow().iter_sections() {
            let row = gtk::ListBoxRow::new();
            let widget = gtk::Label::new(&section.name as &str);
            row.add(&widget);
            
            list.add(&row);
        }

        self.main_notebook.append_page(&list, Some(&gtk::Label::new("Rayons")));
    
    }
    
}
