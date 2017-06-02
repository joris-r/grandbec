extern crate gdk;
extern crate gtk;
use gtk::prelude::*;
use std::fmt;
use std::rc::*;
use std::cell::*;

enum Group {
    Vegetable,
    Fruit,
    Protein,
    Carbohydrate,
    Fat,
    Dairy,
    Cheese,
}

impl fmt::Display for Group {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let printable = match *self {
                Group::Vegetable => "Légume",
                Group::Fruit => "Fruit",
                Group::Protein => "Protéine",
                Group::Carbohydrate => "Féculent",
                Group::Fat => "Graisse",
                Group::Dairy => "Laitage",
                Group::Cheese => "Fromage",
            };
        write!(f, "{}", printable)
    }
}

enum Section {
    Other,
    Greengrocer,
    Frozen,
    Dairy,
    Organic,
    Drink,
    Starchy,
    Condiment,
    Butcher,
    Fishshop,
}

impl fmt::Display for Section {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let printable = match *self {
                Section::Other => "Autre",
                Section::Greengrocer => "Primeur",
                Section::Frozen => "Surgelé",
                Section::Dairy => "Crémerie",
                Section::Organic => "Bio",
                Section::Drink => "Boisson",
                Section::Starchy => "Féculents",
                Section::Condiment => "Condiment",
                Section::Butcher => "Viande",
                Section::Fishshop => "Poisson",
            };
        write!(f, "{}", printable)
    }
}

#[derive(Clone,Copy)]
enum Unit {
    Gram = 0,
    Centilitre,
}

impl fmt::Display for Unit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let printable = match *self {
                Unit::Gram => "g",
                Unit::Centilitre => "cl",
            };
        write!(f, "{}", printable)
    }
}

fn create_combo_of_unit(unit : & Unit) -> gtk::ComboBoxText {
    let c = gtk::ComboBoxText::new();
    c.insert_text(Unit::Gram as i32, &Unit::Gram.to_string());
    c.insert_text(Unit::Centilitre as i32, &Unit::Centilitre.to_string());
    c.set_active(*unit as i32);
    return c;
}

enum Quantity {
    Divisible(u16, Unit),
    Countable(u16)
}

struct Part {
    id : usize,
    name : String,
    group : Group,
    section : Section,
    quantity : Quantity,
}

struct Catalog {
    list : Vec<Part>,
}

fn default_catalog() -> Catalog {
    let mut c = Catalog {
        list : Vec::new()
    };
    
    let p = Part {
        id : 0,
        name : "tomate".to_string(),
        quantity : Quantity::Countable(1),
        group : Group::Vegetable,
        section : Section::Greengrocer,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as usize;
    
    let p = Part {
        id : 0,
        name : "pomme".to_string(),
        quantity : Quantity::Countable(1),
        group : Group::Fruit,
        section : Section::Greengrocer,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as usize;
    
    let p = Part {
        id : 0,
        name : "steak haché".to_string(),
        quantity : Quantity::Countable(1),
        group : Group::Protein,
        section : Section::Butcher,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as usize;
    
    let p = Part {
        id : 0,
        name : "tranche de pain".to_string(),
        quantity : Quantity::Countable(2),
        group : Group::Carbohydrate,
        section : Section::Other,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as usize;
    
    let p = Part {
        id : 0,
        name : "huile".to_string(),
        quantity : Quantity::Divisible(5,Unit::Centilitre),
        group : Group::Fat,
        section : Section::Other,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as usize;
    
    let p = Part {
        id : 0,
        name : "lait".to_string(),
        quantity : Quantity::Divisible(5,Unit::Centilitre),
        group : Group::Dairy,
        section : Section::Dairy,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as usize;
    
    let p = Part {
        id : 0,
        name : "emmental".to_string(),
        quantity : Quantity::Divisible(30,Unit::Gram),
        group : Group::Cheese,
        section : Section::Dairy,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as usize;
    
    return c;
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    
    window.set_title("Grand Bec Project");
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    let catalog : Rc<RefCell<Catalog>> = Rc::new(RefCell::new(default_catalog()));
        
    setup(window.clone(), catalog.clone());
    
    window.show_all();
    gtk::main();
}

fn setup(window : gtk::Window, catalog : Rc<RefCell<Catalog>>) {

    // division par deux panneaux
    let paned = gtk::Paned::new(gtk::Orientation::Horizontal);
    window.add(&paned);

    // partie gauche : le planning
    let left = gtk::ListBox::new();
    paned.add1(&left);
    left.set_size_request(300,300);
    show_planning(&left, catalog.clone());

    // partie droite : le catalogue d'ingrédients
    let right = gtk::ListBox::new();
    paned.add2(&right);
    right.set_size_request(300,300);
    show_catalogue(&right, catalog.clone());
    
}

fn show_planning(target : &gtk::ListBox, catalog : Rc<RefCell<Catalog>>) {
    let days = [
        "lundi",
        "mardi",
        "mercredi",
        "jeudi",
        "vendredi",
        "samedi",
        "dimanche",
    ];
    let lunchs = [
        "petit-déjeuner",
        "déjeuner",
        "gouter",
        "diner",
    ];
    for day in days.iter() {
        for lunch in lunchs.iter() {
            show_empty_lunch(target, &format!("{} {}", day, lunch), catalog.clone());
        }
    }
}

fn show_empty_lunch(target : & gtk::ListBox, label : &str, catalog : Rc<RefCell<Catalog>>) {
    let targets = vec![
        gtk::TargetEntry::new("STRING", gtk::TARGET_SAME_APP, 0),
    ];
    
    // test de drag&drop
    let to = gtk::Label::new(label);
    target.add(&to);
    
    to.drag_dest_set(gtk::DEST_DEFAULT_ALL, &targets, gdk::ACTION_COPY);
    
    let listbox = target.clone();
    
    to.connect_drag_data_received(move |w, _, _, _, s, _, _| {
        let lbr : gtk::ListBoxRow = w
            .get_parent()
            .unwrap()
            .downcast::<gtk::ListBoxRow>()
            .unwrap();
        let drop_pos = lbr.get_index();
        let part_num : usize = s
            .get_text().unwrap()
            .parse().unwrap();
        let part = & catalog.borrow().list[part_num];
        let line = create_part(part);
        listbox.insert(&line, drop_pos+1);
        line.show_all();
    });
}

fn create_part(part : & Part) -> gtk::EventBox {

    let targets = vec![
        gtk::TargetEntry::new("STRING", gtk::TARGET_SAME_APP, 0),
    ];

    let eventbox = gtk::EventBox::new();

    let line = gtk::Grid::new();
    eventbox.add(&line);
    line.set_column_spacing(10);

    let name = gtk::Label::new(&part.name as &str);
    line.add(&name);
    
    eventbox.drag_source_set( gdk::MODIFIER_MASK, &targets, gdk::ACTION_COPY);
    let data = part.id.to_string().clone();
    eventbox.connect_drag_data_get(move |_, _, s, _, _| {
        s.set_text(&data, data.len() as i32);
    });
    
    match part.quantity {
        Quantity::Countable(n) => {
            let spin = gtk::SpinButton::new_with_range(
                1.0, 1000.0, 25.0);
            line.add(&spin);
            spin.set_value(n as f64);
        },
        Quantity::Divisible(n, u) => {
            let spin = gtk::SpinButton::new_with_range(
                1.0, 50.0, 1.0);
            line.add(&spin);
            spin.set_value(n as f64);
            line.add(&create_combo_of_unit(&u));
        },
    };
    
    line.add(&gtk::Label::new(&part.group.to_string() as &str));
    
    let section : String = format!("({})", part.section);
    line.add(&gtk::Label::new(&section as &str));

    return eventbox;
}

fn show_catalogue(target : &gtk::ListBox, catalog : Rc<RefCell<Catalog>>) {

    for p in &catalog.borrow().list {
        let line = create_part(p);
        target.add(&line);
    }
}