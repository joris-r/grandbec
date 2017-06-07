extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

use gui_recipe_list::*;
use gui_unit::*;
use gui_ingr_list::*;

pub struct GuiRecipe {
    data : Rc<RefCell<Data>>,
    main_widget : gtk::Paned,
    recipe : Option<Recipe>,
    grid : gtk::Grid,
    myself : Option<Rc<RefCell<GuiRecipe>>>,
    gui_recipe_list : Rc<RefCell<GuiRecipeList>>,
    catal_ingr : GuiIngrList,
}

impl GuiRecipe {
    pub fn new(data : &Rc<RefCell<Data>>, gui_recipe_list : Rc<RefCell<GuiRecipeList>>) -> GuiRecipe {
        let mut gs = GuiRecipe {
            data : data.clone(),
            main_widget : gtk::Paned::new(gtk::Orientation::Horizontal),
            recipe : None,
            grid : gtk::Grid::new(),
            myself : None,
            gui_recipe_list : gui_recipe_list,
            catal_ingr : GuiIngrList::new(&data.clone()),
        };
        gs.setup();
        gs
    }
    
    pub fn set_myself(&mut self, myself : & Rc<RefCell<GuiRecipe>>) {
        self.myself = Some(myself.clone());
    }
    
    pub fn get_main_widget(&self) -> gtk::Widget {
        self.main_widget.clone().upcast::<gtk::Widget>()
    }
    
    fn setup(&mut self){
    
        self.main_widget.set_wide_handle(true);
        self.grid.set_orientation(gtk::Orientation::Vertical);
        
        self.main_widget.add2(&self.catal_ingr.get_main_widget());
    }
    
    pub fn set_target(&mut self, recipe_id : Id) {
        let x = self.data.borrow();
        let recipe = x.get_recipe(recipe_id).unwrap();
        self.recipe = Some(recipe.clone());
    }
    
    pub fn remove_target(&mut self) {
        self.recipe = None;
    }
        
    pub fn show(&mut self) {
        self.catal_ingr.depopulate();
        match self.recipe {
            None => {
                match self.main_widget.get_child1() {
                    Some(w) => w.destroy(),
                    None => ()
                };
                self.main_widget.add1(&gtk::Label::new("Pas de recette selectionnée"));
            },
            Some(ref recipe) => {
                match self.main_widget.get_child1() {
                    Some(w) => w.destroy(),
                    None => ()
                }
                self.grid = gtk::Grid::new();
                self.main_widget.add1(&self.grid);
                let mut line = 0;
                
                // Modify Button
                let modify_but = gtk::Button::new_with_label("Modifier");
                self.grid.attach(&modify_but, 1, line, 1, 1);
                line += 1;
                
                let hsep = gtk::Separator::new(gtk::Orientation::Horizontal);
                self.grid.attach(&hsep, 0, line, 2, 1);
                line += 1;
                
                // Action: edit a recipe
                let gui_recipe_clone = self.myself.clone().unwrap();
                modify_but.connect_clicked(move |_| {
                    gui_recipe_clone.borrow_mut().edit();
                });
                
                // show name of the recipe
                let name_label = gtk::Label::new(&recipe.name as &str);
                self.grid.attach(&name_label, 0, line, 1, 1);
                line += 1;
                
                let hsep = gtk::Separator::new(gtk::Orientation::Horizontal);
                self.grid.attach(&hsep, 0, line, 2, 1);
                line += 1;
                
                // Ingredients list title
                let ingr_label = gtk::Label::new("Ingrédients");
                self.grid.attach(&ingr_label, 0, line, 1, 1);
                line += 1;
                
                // Ingredients list
                for i in recipe.ingredients.values() {
                    let s = format!("{} {}", i.name, (*i).quantity);
                    let label = gtk::Label::new(&s as &str);
                    self.grid.attach(&label, 0, line, 1, 1);
                    line += 1;
                }
                
                let hsep = gtk::Separator::new(gtk::Orientation::Horizontal);
                self.grid.attach(&hsep, 0, line, 2, 1);
                line += 1;
                
                // Instruction text
                let label_instr = gtk::Label::new("Recette");
                self.grid.attach(&label_instr, 0, line, 1, 1);
                line += 1;
                
                let note = gtk::Label::new(&recipe.note as &str);
                self.grid.attach(&note, 0, line, 1, 1);
//                 line += 1;
                
            },
        }
        // finish
        self.main_widget.show_all();
    }
    
    pub fn edit(&mut self) {
        match self.recipe {
            None => {
                match self.main_widget.get_child1() {
                    Some(w) => w.destroy(),
                    None => ()
                };
                self.main_widget.add1(&gtk::Label::new("Pas de recette selectionnée"));
            },
            Some(ref mut recipe) => {
                match self.main_widget.get_child1() {
                    Some(w) => w.destroy(),
                    None => ()
                }
                self.grid = gtk::Grid::new();
                self.main_widget.add1(&self.grid);
                let mut line = 0;
                
                // Cancel Button
                let cancel_but = gtk::Button::new_with_label("Annuler");
                self.grid.attach(&cancel_but, 0, line, 1, 1);
                
                // Action: cancel the recipe edition
                let gui_recipe_clone = self.myself.clone().unwrap();
                let recipe_id = recipe.id;
                cancel_but.connect_clicked(move |_| {
                    gui_recipe_clone.borrow_mut().set_target(recipe_id);
                    gui_recipe_clone.borrow_mut().show();
                });
                
                // Accept Button
                let accept_but = gtk::Button::new_with_label("Valider");
                self.grid.attach(&accept_but, 1, line, 1, 1);
                line+=1;
                
                // show name of the recipe
                self.grid.attach(&gtk::Label::new("Nom : "), 0, line, 1, 1);
                let name_eb = gtk::EntryBuffer::new(Some(&recipe.name as &str));
                let name_e = gtk::Entry::new_with_buffer(&name_eb);
                self.grid.attach(&name_e, 1, line, 1, 1);
                line+=1;
                
                // Action: take into account renaming
                let gui_recipe_clone = self.myself.clone().unwrap();
                let name_eb_clone = name_eb.clone();
                name_e.connect_changed(move |_| {
                    gui_recipe_clone.borrow_mut().recipe.as_mut().unwrap().name =
                        name_eb_clone.get_text();
                });
                
                // Ingredients list title
                let ingr_label = gtk::Label::new("Ingrédients");
                self.grid.attach(&ingr_label, 0, line, 1, 1);
                line += 1;
                
                // Ingredients list
                let sub_grid = gtk::Grid::new();
                let mut sub_line = 0;
                self.grid.attach(&sub_grid, 0, line, 2, 1);
                line += 1;
                for i in recipe.ingredients.values() {
                
                    // ingredient removing
                    let rm_but = gtk::Button::new_with_label("X");
                    sub_grid.attach(&rm_but, 0, sub_line, 1, 1);
                    
                    let gui_recipe_clone = self.myself.clone().unwrap();
                    let i_id = i.id;
                    rm_but.connect_clicked(move |_| {
                        gui_recipe_clone.borrow_mut().recipe.as_mut().unwrap()
                            .ingredients.remove(&i_id);
                        gui_recipe_clone.borrow_mut().edit()
                    });
                    
                    // ingredient name
                    let i_name_label = gtk::Label::new(&i.name as &str);
                    sub_grid.attach(&i_name_label, 1, sub_line, 1, 1);
                    
                    //ingredient numeric quantity
                    let qty = gtk::SpinButton::new_with_range(
                        1.0, 1000.0, 1.0);
                    sub_grid.attach(&qty, 2, sub_line, 1, 1);
                    qty.set_digits(1);
                    qty.set_value((*i).quantity.val);
                    
                    // store quantity change
                    let gui_recipe_clone = self.myself.clone().unwrap();
                    let i_id = i.id;
                    qty.connect_value_changed(move |w| {
                        gui_recipe_clone.borrow_mut().recipe.as_mut().unwrap()
                            .ingredients.get_mut(&i_id).unwrap()
                            .quantity.val = w.get_value();
                    });
                    
                    let unit = create_combo_of_unit(&i.quantity.unit);
                    sub_grid.attach(&unit, 3, sub_line, 1, 1);
                    
                    // store unit change
                    let gui_recipe_clone = self.myself.clone().unwrap();
                    let i_id = i.id;
                    unit.connect_changed(move |w| {
                        gui_recipe_clone.borrow_mut().recipe.as_mut().unwrap()
                            .ingredients.get_mut(&i_id).unwrap()
                            .quantity.unit = read_unit(&w);
                    });
                    
                    sub_line += 1;
                }
                
                // Button add ingredient
                let add_ingr_but = gtk::Button::new_with_label("Ajouter ingrédient");
                self.grid.attach(&add_ingr_but, 0, line, 2, 1);
                line += 1;
                
                let gui_recipe_clone = self.myself.clone().unwrap();
                add_ingr_but.connect_clicked(move |_| {
                    let x = gui_recipe_clone.borrow();
                    x.catal_ingr.populate_ingr(&gui_recipe_clone);
                });
                
                // Instruction text
                let note_frame = gtk::Frame::new("Recette");
                self.grid.attach(&note_frame, 0, line, 2, 1);
                note_frame.set_size_request(300,150);
                
                let tb = gtk::TextBuffer::new(None);
                tb.set_text(&recipe.note as &str);
                let tv = gtk::TextView::new_with_buffer(&tb);
                note_frame.add(&tv);
                
                // Action: accept modification
                let gui_recipe_clone = self.myself.clone().unwrap();
                let gui_recipe_list_clone = self.gui_recipe_list.clone();
                let data_clone = self.data.clone();
                let tb_clone = tb.clone();
                accept_but.connect_clicked(move |_| {
                    let new_note = tb_clone.get_text(
                        &tb_clone.get_start_iter(),
                        &tb_clone.get_end_iter(),
                        false).unwrap();
                    let mut new_recipe = gui_recipe_clone.borrow().recipe.clone().unwrap();
                    new_recipe.note = new_note;
                    data_clone.borrow_mut().add_recipe(&new_recipe);
                    gui_recipe_clone.borrow_mut().set_target(new_recipe.id);
                    gui_recipe_clone.borrow_mut().show();
                    gui_recipe_list_clone.borrow_mut().update_list();
                });
                line += 1;
                
                // Delete Button
                let del_but = gtk::Button::new_with_label("Supprimer");
                self.grid.attach(&del_but, 0, line, 2, 1);
                
                let data_clone = self.data.clone();
                let gui_recipe_clone = self.myself.clone().unwrap();
                let gui_recipe_list_clone = self.gui_recipe_list.clone();
                del_but.connect_clicked(move |_| {
                    data_clone.borrow_mut().remove_recipe(recipe_id);
                    gui_recipe_clone.borrow_mut().remove_target();
                    gui_recipe_clone.borrow_mut().show();
                    gui_recipe_list_clone.borrow_mut().update_list();
                });
    
            },
        }
        // finish
        self.main_widget.show_all();
    }
    
    pub fn add_ingredient(&mut self, ingr : &Ingredient) {
        match self.recipe {
            None => {},
            Some(ref mut recipe) => {
                recipe.ingredients.insert(ingr.id, ingr.clone());
            }
        }
        self.catal_ingr.depopulate();
        self.edit();
    }

}
