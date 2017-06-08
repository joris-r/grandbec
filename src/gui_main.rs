extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

use gui_book::*;
use gui_goods::*;

pub struct GuiMain {
    notebook : gtk::Notebook,
    gui_book : GuiBook,
    gui_goods : GuiGoods,
}

impl GuiMain {
    pub fn new(data : &Rc<RefCell<Data>>) -> GuiMain {

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_size_request(640,480);
        
        window.set_title("Grand Bec Project");
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        
        let mut gs = GuiMain {
            notebook : gtk::Notebook::new(),
            gui_book : GuiBook::new(&data),
            gui_goods : GuiGoods::new(&data),
        };
        window.add(&gs.notebook);
        gs.setup();
        window.show_all();
        gtk::main();
        gs
    }
    
    fn setup(&mut self){
        self.notebook.append_page(
            &self.gui_goods.get_main_widget(),
            Some(&gtk::Label::new("Articles")));
        
        self.notebook.append_page(
            &self.gui_book.get_main_widget(),
            Some(&gtk::Label::new("Recettes")));
        
        self.notebook.append_page(
            &gtk::Label::new("TODO"),
            Some(&gtk::Label::new("Menu")));
        
        self.notebook.append_page(
            &gtk::Label::new("TODO"),
            Some(&gtk::Label::new("Courses")));
        
        self.notebook.append_page(
            &gtk::Label::new("TODO"),
            Some(&gtk::Label::new("Historique")));

    }
    
}
