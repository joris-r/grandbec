extern crate gdk;
extern crate gtk;
use gtk::prelude::*;
use std::fmt;

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
    id : u32,
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
    c.list.last_mut().unwrap().id = c.list.len() as u32;
    
    let p = Part {
        id : 0,
        name : "pomme".to_string(),
        quantity : Quantity::Countable(1),
        group : Group::Fruit,
        section : Section::Greengrocer,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as u32;
    
    let p = Part {
        id : 0,
        name : "steak haché".to_string(),
        quantity : Quantity::Countable(1),
        group : Group::Protein,
        section : Section::Butcher,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as u32;
    
    let p = Part {
        id : 0,
        name : "tranche de pain".to_string(),
        quantity : Quantity::Countable(2),
        group : Group::Carbohydrate,
        section : Section::Other,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as u32;
    
    let p = Part {
        id : 0,
        name : "huile".to_string(),
        quantity : Quantity::Divisible(5,Unit::Centilitre),
        group : Group::Fat,
        section : Section::Other,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as u32;
    
    let p = Part {
        id : 0,
        name : "lait".to_string(),
        quantity : Quantity::Divisible(5,Unit::Centilitre),
        group : Group::Dairy,
        section : Section::Dairy,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as u32;
    
    let p = Part {
        id : 0,
        name : "emmental".to_string(),
        quantity : Quantity::Divisible(30,Unit::Gram),
        group : Group::Cheese,
        section : Section::Dairy,
    };
    c.list.push(p);
    c.list.last_mut().unwrap().id = c.list.len() as u32;
    
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
    
    let catalog = default_catalog();
    
    setup(window.clone(), &catalog);
    
    window.show_all();
    gtk::main();
}

fn setup(window : gtk::Window, catalog : &Catalog) {

    // division par deux panneaux
    let paned = gtk::Paned::new(gtk::Orientation::Horizontal);
    window.add(&paned);

    // partie gauche : le planning
//     let left = gtk::ListBox::new();
    let left = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    paned.add1(&left);
    left.set_size_request(300,300);

    // partie droite : le catalogue d'ingrédients
    let right = gtk::ListBox::new();
    paned.add2(&right);
    right.set_size_request(300,300);
    show_catalogue(&right, catalog);
    
    // test de drag&drop
    let to = gtk::Label::new("Drop here");
    
    left.pack_start(&to, true, true, 0);
    
    let targets = vec![
        gtk::TargetEntry::new("STRING", gtk::TARGET_SAME_APP, 0),
    ];
    
    to.drag_dest_set(gtk::DEST_DEFAULT_ALL, &targets, gdk::ACTION_COPY);
    to.connect_drag_data_received(|w, _, _, _, s, _, _| {
        w.set_text(&s.get_text().unwrap());
    });

}

fn show_catalogue(target : &gtk::ListBox, catalog : &Catalog) {

    let targets = vec![
        gtk::TargetEntry::new("STRING", gtk::TARGET_SAME_APP, 0),
    ];

    for p in &catalog.list {
        let eventbox = gtk::EventBox::new();
        target.add(&eventbox);
    
        let line = gtk::Grid::new();
        eventbox.add(&line);
        line.set_column_spacing(10);

        let name = gtk::Label::new(&p.name as &str);
        line.add(&name);
        
        eventbox.drag_source_set( gdk::MODIFIER_MASK, &targets, gdk::ACTION_COPY);
        let data = p.id.to_string().clone();
        eventbox.connect_drag_data_get(move |_, _, s, _, _| {
            s.set_text(&data, data.len() as i32);
        });
        
        match p.quantity {
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
        
        line.add(&gtk::Label::new(&p.group.to_string() as &str));
        
        let section : String = format!("({})", p.section);
        line.add(&gtk::Label::new(&section as &str));
    }
}
