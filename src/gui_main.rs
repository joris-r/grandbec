extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

use gui_book::*;
use gui_goods::*;

pub struct GuiMain {
    window : gtk::Window,
    notebook : gtk::Notebook,
    gui_book : GuiBook,
    gui_goods : Rc<RefCell<GuiGoods>>,
}

impl GuiMain {
    pub fn new(data : &Rc<RefCell<Data>>) -> GuiMain {
        let gui_main = GuiMain {
            window : gtk::Window::new(gtk::WindowType::Toplevel),
            notebook : gtk::Notebook::new(),
            gui_book : GuiBook::new(&data),
            gui_goods : Rc::new(RefCell::new(GuiGoods::new(&data))),
        };
        gui_main.gui_goods.borrow_mut().set_myself(&gui_main.gui_goods);
        gui_main
    }
    
    pub fn setup(&mut self){
        self.window.set_size_request(640,480);
        self.window.set_title("Grand Bec Project");
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        
        self.gui_book.setup();
        self.gui_goods.borrow_mut().setup();
        
        self.window.add(&self.notebook);
        self.notebook.append_page(
            &self.gui_goods.borrow().get_main_widget(),
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
            
        self.window.show_all();
        gtk::main();

    }
    
}
