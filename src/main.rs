extern crate gtk;
use gtk::prelude::*;

// extern crate gdk;

use std::rc::*;
use std::cell::*;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod logic; use logic::*;
mod file; use file::*;

fn main(){
    let data : Rc<RefCell<Data>> = Rc::new(RefCell::new(Data::new()));
    load_all(&mut data.borrow_mut(), std::path::Path::new("ressources"));
    main_gui(data.clone());
    save_all(& data.borrow(), std::path::Path::new("."));
}

fn main_gui(data : Rc<RefCell<Data>>) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_size_request(640,480);
    
    window.set_title("Grand Bec Project");
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
        
    setup(window.clone(), data.clone());
    
    window.show_all();
    gtk::main();
}

fn setup(window : gtk::Window, data : Rc<RefCell<Data>>) {

    let notebook = gtk::Notebook::new();
    window.add(&notebook);
    
    let book_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    book_pane.set_wide_handle(true);
    notebook.append_page(&book_pane, Some(&gtk::Label::new("Recettes")));
    
    let recipies_list = gtk::ListBox::new();
    
    // TODO bug: doesn't work
    recipies_list.set_activate_on_single_click(true);
    // anyway the propertie was true already
    
    recipies_list.set_selection_mode(gtk::SelectionMode::None);
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
        show_recipe_edit(&data_clone, &book_pane_clone , recipe_id);
    });
    
    // Ingredients list
    grid.add(&gtk::Label::new("Ingrédients"));
    
    for i in &recipe.ingredients {
        let s = format!("{} {}", i.quantity, i.name);
        grid.add(&gtk::Label::new(&s as &str));
    }
    
    // Instruction text
    let note_frame = gtk::Frame::new("Instructions");
    grid.add(&note_frame);
    note_frame.set_size_request(300,150);
    note_frame.add(&gtk::Label::new(&recipe.note as &str));

    frame.show_all();
}

fn show_recipe_edit(data : &Rc<RefCell<Data>>, book_pane : &gtk::Paned, recipe_id : Id) {

    data.borrow_mut().clone_into_edited_recipe(recipe_id);

    let recipe = data.borrow().get_recipe(recipe_id).unwrap().clone();
    book_pane.get_child2().unwrap().destroy();
    
    let edit_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    book_pane.add2(&edit_pane);
    edit_pane.set_wide_handle(true);
    
    let frame = gtk::Frame::new(Some(&recipe.name as &str));
    edit_pane.add1(&frame);
    
    let ingredients = give_ingredients_list(data);
    edit_pane.add2(&ingredients);
    
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
    
    // Ingredients list
    grid.add(&gtk::Label::new("Ingrédients"));
    
    for i in &recipe.ingredients {
        let line = gtk::Grid::new();
        grid.add(&line);
        
        let qty = gtk::SpinButton::new_with_range(
            1.0, 1000.0, 1.0);
        line.add(&qty);
        qty.set_digits(1);
        qty.set_value(i.quantity.val);
        
//         let data_clone = data.clone();
//         qty.connect_value_changed(|w| {
            // TODO
//         };
        
        let unit = create_combo_of_unit(&i.quantity.unit);
        line.add(&unit);
        
        line.add(&gtk::Label::new(&i.name as &str));
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

fn give_ingredients_list(data : &Rc<RefCell<Data>>) -> gtk::ListBox {
    let list = gtk::ListBox::new();
    
    for i in data.borrow().iter_ingredients() {
        let row = gtk::ListBoxRow::new();
        list.add(&row);
        let widget = gtk::Label::new(&i.name as &str);
        row.add(&widget);
    }
    
    list
}

fn create_combo_of_unit(unit : & Unit) -> gtk::ComboBoxText {
    let c = gtk::ComboBoxText::new();
    c.insert_text(Unit::Portion as i32, &Unit::Portion.to_string());
    c.insert_text(Unit::Gram as i32, &Unit::Gram.to_string());
    c.insert_text(Unit::Centilitre as i32, &Unit::Centilitre.to_string());
    c.set_active(*unit as i32);
    return c;
}


// fn show_empty_lunch(target : & gtk::ListBox, label : &str, catalog : Rc<RefCell<Catalog>>) {
//     let targets = vec![
//         gtk::TargetEntry::new("STRING", gtk::TARGET_SAME_APP, 0),
//     ];
//     
//     // test de drag&drop
//     let to = gtk::Label::new(label);
//     target.add(&to);
//     
//     to.drag_dest_set(gtk::DEST_DEFAULT_ALL, &targets, gdk::ACTION_COPY);
//     
//     let listbox = target.clone();
//     
//     to.connect_drag_data_received(move |w, _, _, _, s, _, _| {
//         let lbr : gtk::ListBoxRow = w
//             .get_parent()
//             .unwrap()
//             .downcast::<gtk::ListBoxRow>()
//             .unwrap();
//         let drop_pos = lbr.get_index();
//         let part_num : usize = s
//             .get_text().unwrap()
//             .parse().unwrap();
//         let part = & catalog.borrow().list[part_num];
//         let line = create_part(part);
//         listbox.insert(&line, drop_pos+1);
//         line.show_all();
//     });
// }

// fn create_part(part : & Part) -> gtk::EventBox {
// 
//     let targets = vec![
//         gtk::TargetEntry::new("STRING", gtk::TARGET_SAME_APP, 0),
//     ];
// 
//     let eventbox = gtk::EventBox::new();
// 
//     let line = gtk::Grid::new();
//     eventbox.add(&line);
//     line.set_column_spacing(10);
// 
//     let name = gtk::Label::new(&part.name as &str);
//     line.add(&name);
//     
//     eventbox.drag_source_set( gdk::MODIFIER_MASK, &targets, gdk::ACTION_COPY);
//     let data = part.id.to_string().clone();
//     eventbox.connect_drag_data_get(move |_, _, s, _, _| {
//         s.set_text(&data, data.len() as i32);
//     });
//     
//     match part.quantity {
//         Quantity::Countable(n) => {
//             let spin = gtk::SpinButton::new_with_range(
//                 1.0, 1000.0, 25.0);
//             line.add(&spin);
//             spin.set_value(n as f64);
//         },
//         Quantity::Divisible(n, u) => {
//             let spin = gtk::SpinButton::new_with_range(
//                 1.0, 50.0, 1.0);
//             line.add(&spin);
//             spin.set_value(n as f64);
//             line.add(&create_combo_of_unit(&u));
//         },
//     };
//     
//     line.add(&gtk::Label::new(&part.group.to_string() as &str));
//     
//     let section : String = format!("({})", part.section);
//     line.add(&gtk::Label::new(&section as &str));
// 
//     return eventbox;
// }
