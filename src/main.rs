extern crate gtk;

// extern crate gdk;

use std::rc::*;
use std::cell::*;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod logic; use logic::*;
mod file; use file::*;
// mod gui_planning;
// mod gui_ingr;
mod gui_main; use gui_main::*;
mod gui_book;
mod gui_recipe_list;
mod gui_recipe;
mod gui_unit;
mod gui_ingr_list;

fn main(){
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
        
    let data : Rc<RefCell<Data>> = Rc::new(RefCell::new(Data::new()));
    load_all(&mut data.borrow_mut(), std::path::Path::new("ressources"));
    
    let _main_gui = GuiMain::new(&data);
    
    save_all(& data.borrow(), std::path::Path::new("."));
}
