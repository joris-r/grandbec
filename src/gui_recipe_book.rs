extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;

pub fn setup_recipe_book(notebook : &gtk::Notebook, data : &Rc<RefCell<Data>>) {

    let recipe_zone = gtk::Grid::new();
    notebook.append_page(&recipe_zone, Some(&gtk::Label::new("Recettes")));
    recipe_zone.set_orientation(gtk::Orientation::Vertical);
    
    let new_but = gtk::Button::new_with_label("+");
    recipe_zone.add(&new_but);

    let notebook_clone = notebook.clone();
    let data_clone = data.clone();
    let recipe_zone_clone = recipe_zone.clone();
    new_but.connect_clicked(move |_| {
        data_clone.borrow_mut().add_recipe( & Recipe::new(
            "recette",
            "",
        ));
        recipe_zone_clone.destroy();
        setup_recipe_book(&notebook_clone, &data_clone);
    });

    
    let book_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    book_pane.set_wide_handle(true);
    recipe_zone.add(&book_pane);
    
    let recipies_list = gtk::ListBox::new();
    
    // workaround for the activation problem
    recipies_list.set_selection_mode(gtk::SelectionMode::Single);
    recipies_list.connect_row_selected(move |_,olbr| {
        match olbr {
            &Some(ref lbr) => {lbr.activate();},
            _ => {},
        }
    });
    
    book_pane.add1(&recipies_list);
    book_pane.add2(&gtk::Label::new("Pas de recette selectionnée"));

    for recipe in data.borrow().iter_recipes() {
        let row = gtk::ListBoxRow::new();
        recipies_list.add(&row);
        let widget = gtk::Label::new(&recipe.name as &str);
        row.add(&widget);
        
        let book_pane_clone = book_pane.clone();
        let recipe_id = recipe.id;
        let data_clone = data.clone();
        row.connect_activate(move |_| {
            show_recipe_content(&data_clone, &book_pane_clone, recipe_id);
        });
        
    }
    
    recipe_zone.show_all();
}

fn show_recipe_content(data : &Rc<RefCell<Data>>, book_pane : &gtk::Paned, recipe_id : Id) {
    let dref = data.borrow();
    let recipe = dref.get_recipe(recipe_id).unwrap();
    book_pane.get_child2().unwrap().destroy();
    
    let frame = gtk::Frame::new(Some(&recipe.name as &str));
    book_pane.add2(&frame);
    
    let grid = gtk::Grid::new();
    frame.add(&grid);
    grid.set_orientation(gtk::Orientation::Vertical);
    
    // Modify Button
    let modify_but = gtk::Button::new_with_label("Modifier");
    grid.add(&modify_but);
    
    let book_pane_clone = book_pane.clone();
    let data_clone = data.clone();
    modify_but.connect_clicked(move |_| {
        data_clone.borrow_mut().clone_into_edited_recipe(recipe_id);
        show_recipe_edit(&data_clone, &book_pane_clone);
    });
    
    // Delete Button
    let del_but = gtk::Button::new_with_label("Supprimer");
    grid.add(&del_but);
    
    let book_pane_clone = book_pane.clone();
    let data_clone = data.clone();
    del_but.connect_clicked(move |_| {
        data_clone.borrow_mut().remove_recipe(recipe_id);
        book_pane_clone.get_child2().unwrap().destroy();
        book_pane_clone.add2(&gtk::Label::new("Recette suprimée"));
        book_pane_clone.show_all();
    });
    
    // Ingredients list
    grid.add(&gtk::Label::new("Ingrédients"));
    
    for i in recipe.ingredients.values() {
        let s = format!("{} {}", (*i).quantity, i.name);
        grid.add(&gtk::Label::new(&s as &str));
    }
    
    // Instruction text
    let note_frame = gtk::Frame::new("Instructions");
    grid.add(&note_frame);
    note_frame.set_size_request(300,150);
    note_frame.add(&gtk::Label::new(&recipe.note as &str));

    frame.show_all();
}

// don't forget to set up data into Data.edited_recipe before call
fn show_recipe_edit(data : &Rc<RefCell<Data>>, book_pane : &gtk::Paned) {

    book_pane.get_child2().unwrap().destroy();
    
    let recipe = data.borrow().edited_recipe.clone();
    let recipe_id = data.borrow().edited_recipe.id;

    
    let edit_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    book_pane.add2(&edit_pane);
    edit_pane.set_wide_handle(true);
    
    let frame = gtk::Frame::new(Some(&recipe.name as &str));
    edit_pane.add1(&frame);
    
    // Ingredients Catalog on the right pane
    let list_ingr = gtk::ListBox::new();
    edit_pane.add2(&list_ingr);
    
    // workaround for the activation problem
    list_ingr.set_selection_mode(gtk::SelectionMode::Single);
    list_ingr.connect_row_selected(move |_,olbr| {
        match olbr {
            &Some(ref lbr) => {lbr.activate();},
            _ => {},
        }
    });
    
    for i in data.borrow().iter_ingredients() {
        let row = gtk::ListBoxRow::new();
        list_ingr.add(&row);
        let widget = gtk::Label::new(&i.name as &str);
        row.add(&widget);
        
        // add new ingredient to the recipe
        let data_clone = data.clone();
        let new_ingr_id = i.id;
        let book_pane_clone = book_pane.clone();
        row.connect_activate(move |_| {
            let ingr = data_clone.borrow()
                .get_ingredient(new_ingr_id).unwrap().clone();
            data_clone.borrow_mut().edited_recipe
                .ingredients.insert(new_ingr_id, ingr);
            // redraw all
            show_recipe_edit(&data_clone, &book_pane_clone);
        });
        
    }
    
    let grid = gtk::Grid::new();
    frame.add(&grid);
    grid.set_orientation(gtk::Orientation::Vertical);
    
    // Cancel Button
    let cancel_but = gtk::Button::new_with_label("Annuler");
    grid.add(&cancel_but);

    let book_pane_clone = book_pane.clone();
    let data_clone = data.clone();
    cancel_but.connect_clicked(move |_| {
        show_recipe_content(&data_clone, &book_pane_clone , recipe_id);
    });
    
    // Accept Button
    let valid_but = gtk::Button::new_with_label("Valider");
    grid.add(&valid_but);
    
    // renaming
    let name_eb = gtk::EntryBuffer::new(Some(&recipe.name as &str));
    let name_e = gtk::Entry::new_with_buffer(&name_eb);
    grid.add(&name_e);
    
    let data_clone = data.clone();
    let name_eb_clone = name_eb.clone();
    name_e.connect_changed(move |_| {
        data_clone.borrow_mut().edited_recipe.name =
            name_eb_clone.get_text();
    });
    
    // Ingredients list
    grid.add(&gtk::Label::new("Ingrédients"));
    
    for i in recipe.ingredients.values() {
        let line = gtk::Grid::new();
        grid.add(&line);
        
        let qty = gtk::SpinButton::new_with_range(
            1.0, 1000.0, 1.0);
        line.add(&qty);
        qty.set_digits(1);
        qty.set_value((*i).quantity.val);
        
        // store quantity change
        let data_clone = data.clone();
        let i_id = i.id;
        qty.connect_value_changed(move |w| {
            data_clone.borrow_mut().edited_recipe
                .ingredients.get_mut(&i_id).unwrap()
                .quantity.val = w.get_value();
        });
        
        let unit = create_combo_of_unit(&i.quantity.unit);
        line.add(&unit);
        
        // store unit change
        let data_clone = data.clone();
        let i_id = i.id;
        unit.connect_changed(move |w| {
            data_clone.borrow_mut().edited_recipe
                .ingredients.get_mut(&i_id).unwrap()
                .quantity.unit = read_unit(&w);
        });
        
        line.add(&gtk::Label::new(&i.name as &str));
        
        // ingredient removing
        let rm_but = gtk::Button::new_with_label("X");
        line.add(&rm_but);
        
        let data_clone = data.clone();
        let i_id = i.id;
        let book_pane_clone = book_pane.clone();
        rm_but.connect_clicked(move |_| {
            data_clone.borrow_mut().edited_recipe
                .ingredients.remove(&i_id);
            // redraw all
            show_recipe_edit(&data_clone, &book_pane_clone);
        });
        
    }
    
    // Instruction text
    let note_frame = gtk::Frame::new("Instructions");
    grid.add(&note_frame);
    note_frame.set_size_request(300,150);
    
    let tb = gtk::TextBuffer::new(None);
    tb.set_text(&recipe.note as &str);
    let tv = gtk::TextView::new_with_buffer(&tb);
    note_frame.add(&tv);
    
    // Accept Button action
    let book_pane_clone = book_pane.clone();
    let data_clone = data.clone();
    let tb_clone = tb.clone();
    valid_but.connect_clicked(move |_| {
        let new_note = tb_clone.get_text(
            &tb_clone.get_start_iter(),
            &tb_clone.get_end_iter(),
            false).unwrap();
        data_clone.borrow_mut().edited_recipe.note = new_note;
        data_clone.borrow_mut().save_edited_recipe();
        show_recipe_content(&data_clone, &book_pane_clone , recipe_id);
    });
    
    // finish
    edit_pane.show_all();
}

fn create_combo_of_unit(unit : & Unit) -> gtk::ComboBoxText {
    let c = gtk::ComboBoxText::new();
    c.insert_text(Unit::Portion as i32, &Unit::Portion.to_string());
    c.insert_text(Unit::Gram as i32, &Unit::Gram.to_string());
    c.insert_text(Unit::Centilitre as i32, &Unit::Centilitre.to_string());
    c.set_active(*unit as i32);
    return c;
}

fn read_unit(widget : &gtk::ComboBoxText) -> Unit {
    let val = widget.get_active();
    if val == Unit::Portion as i32 { Unit::Portion }
    else if val == Unit::Gram as i32 { Unit::Gram }
    else if val == Unit::Centilitre as i32 { Unit::Centilitre }
    else { panic!("wrong Unit id") }
}
