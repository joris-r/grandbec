extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

use gui_recipe_list::*;
use gui_recipe::*;

pub struct GuiBook {
    data : Rc<RefCell<Data>>,
    main_widget : gtk::Paned,
    gui_recipes_list : Rc<RefCell<GuiRecipeList>>,
    gui_recipe : Rc<RefCell<GuiRecipe>>,
}

impl GuiBook {
    pub fn new(data : &Rc<RefCell<Data>>) -> GuiBook {
        let grl = Rc::new(RefCell::new(GuiRecipeList::new(data)));
        let mut gs = GuiBook {
            data : data.clone(),
            main_widget : gtk::Paned::new(gtk::Orientation::Horizontal),
            gui_recipes_list : grl.clone(),
            gui_recipe : Rc::new(RefCell::new(GuiRecipe::new(data, grl))),
        };
        gs.gui_recipe.borrow_mut().set_myself(&gs.gui_recipe);
        gs.setup();
        gs
    }
    
    pub fn get_main_widget(&self) -> gtk::Widget {
        self.main_widget.clone().upcast::<gtk::Widget>()
    }
    
    fn setup(&mut self){
    
        self.main_widget.set_wide_handle(true);
        
        self.main_widget.add1(&self.gui_recipes_list.borrow().get_main_widget());
        self.main_widget.add2(&self.gui_recipe.borrow().get_main_widget());
        
        // populate recipe list
        let x = self.data.clone();
        for recipe in x.borrow().iter_recipes() {
            self.gui_recipes_list.borrow_mut()
                .list_recipe(recipe, &self.gui_recipe);
        }
        
        // Action: add a new recipe
        let gui_recipes_list_clone = self.gui_recipes_list.clone();
        let gui_recipe_clone = self.gui_recipe.clone();
        let new_but = self.gui_recipes_list.borrow().new_but.clone();
        new_but.connect_clicked(move |_| {
            let recipe = Recipe::new("Nouvelle recette", "", );
            
            let mut x = gui_recipes_list_clone.borrow_mut();
            x.data.borrow_mut().add_recipe(&recipe);
            
            x.list_recipe(&recipe, &gui_recipe_clone);
            gui_recipe_clone.borrow_mut().set_target(recipe.id);
            gui_recipe_clone.borrow_mut().edit();
        });

    }
    
}
