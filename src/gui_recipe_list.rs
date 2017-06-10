extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;
use std::collections::HashMap;

use logic::*;
use gui_recipe::*;

pub struct GuiRecipeList {
    pub data : Rc<RefCell<Data>>,
    recipes_list : gtk::ListBox,
    pub new_but : gtk::Button,
    info : HashMap<Id, gtk::Label>,
}

impl GuiRecipeList {
    pub fn new(data : &Rc<RefCell<Data>>) -> GuiRecipeList {
        let gs = GuiRecipeList {
            data : data.clone(),
            recipes_list : gtk::ListBox::new(),
            new_but : gtk::Button::new_with_label("Ajouter"),
            info : HashMap::new(),
        };
        gs
    }
    
    pub fn get_main_widget(&self) -> gtk::Widget {
        self.recipes_list.clone().upcast::<gtk::Widget>()
    }
    
    pub fn setup(&mut self){
        
        // workaround for the activation problem
        self.recipes_list.set_selection_mode(gtk::SelectionMode::Single);
        self.recipes_list.connect_row_selected(move |_,olbr| {
            match olbr {
                &Some(ref lbr) => {lbr.activate();},
                _ => {},
            }
        });
        
        // add recipe button
        self.recipes_list.add(&self.new_but);
        
    }
    
    pub fn update_list(&mut self) {
        let x = self.data.borrow();
        let mut to_del = vec![];
        for (id, label) in self.info.iter() {
            match x.get_recipe(*id) {
                Some(recipe) => {
                    label.set_label(&recipe.name as &str);
                },
                None => {
                    label.get_parent().unwrap().destroy();
                    to_del.push(*id);
                }
            }
        }
        for id in to_del {
            self.info.remove(&id);
        }
    }
    
    pub fn list_recipe(&mut self, recipe : &Recipe, gui_recipe : &Rc<RefCell<GuiRecipe>>) {
        let row = gtk::ListBoxRow::new();
        self.recipes_list.add(&row);
        let widget = gtk::Label::new(&recipe.name as &str);
        row.add(&widget);
        row.show_all();
        
        // remember ids
        self.info.insert(recipe.id, widget.clone());
        
        // Action: show selected recipe
        let gui_recipe_clone = gui_recipe.clone();
        let recipe_id = recipe.id;
        row.connect_activate(move |_| {
            gui_recipe_clone.borrow_mut().set_target(recipe_id);
            gui_recipe_clone.borrow_mut().show();
        });
        
    }
    
}
