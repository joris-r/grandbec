
use std::fmt;


struct Data {
    catalog : Catalog,
    book : Book,
    planning : Planning,
    history : History,
    stock : Stock,
}

pub struct Catalog {
    pub list : Vec<Part>,
}

struct Book {
}

struct Planning {
}

struct History {
}

struct Stock {
}

// struct Id { u32 };
// 
// impl Id {
//     new() -> Id {
//         Id(22)
//     }
// }

pub struct Part {
    pub id : usize,
    pub name : String,
    pub group : Group,
    pub section : Section,
    pub quantity : Quantity,
}



pub enum Group {
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

pub enum Section {
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
pub enum Unit {
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

pub enum Quantity {
    Divisible(u16, Unit),
    Countable(u16)
}

pub fn default_catalog() -> Catalog {
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
