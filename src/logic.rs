
#![allow(dead_code)]


extern crate rand;

use std::fmt;
use std::collections::HashMap;

pub struct Data {
    catalog : Catalog,
    book : Book,
    planning : Planning,
    history : History,
    stock : Stock,
}

impl Data {
    pub fn new() -> Data {
        Data {
            catalog : Catalog {
                ingredients : HashMap::new(),
                sections : HashMap::new(),
                groups : HashMap::new(),
            },
            book : Book {},
            planning : Planning {},
            history : History {},
            stock : Stock {},
        }
    }
    
    pub fn add_section(&mut self, section : &Section) {
        // TODO check if id is unique
        self.catalog.sections.insert(section.id, section.clone());
    }
    pub fn add_group(&mut self, group : &Group) {
        // TODO check if id is unique
        self.catalog.groups.insert(group.id, group.clone());
    }
    pub fn add_ingredient(&mut self, ingredient : &Ingredient) {
        // TODO check if id is unique
        self.catalog.ingredients.insert(ingredient.id, ingredient.clone());
    }
}
        
struct Catalog {
    ingredients : HashMap<Id,Ingredient>,
    sections : HashMap<Id,Section>,
    groups : HashMap<Id,Group>,
}


struct Book {
}

struct Planning {
}

struct History {
}

struct Stock {
}

#[derive(Clone)]
pub struct Ingredient {
    pub id : Id,
    pub name : String,
    pub group : Group,
    pub section : Section,
    pub quantity : Quantity,
}

impl Ingredient {
    pub fn new(
        name : &str,
        group : &Group,
        section : &Section,
        quantity : &Quantity
    ) -> Ingredient {
        Ingredient {
            id : Id::new(),
            name : name.to_string(),
            group : group.clone(),
            section : section.clone(),
            quantity : quantity.clone(),
        }
    }
}


#[derive(Clone)]
pub struct Group {
    id : Id,
    name : String,
}

impl Group {
    pub fn new(name : &str) -> Group {
        Group{
            id : Id::new(),
            name : name.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Section {
    id : Id,
    name : String,
}

impl Section {
    pub fn new(name : &str) -> Section {
        Section{
            id : Id::new(),
            name : name.to_string(),
        }
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone,Copy)]
pub struct Quantity {
    val : f64,
    unit : Unit,
}


#[derive(Clone,Copy)]
pub enum Unit {
    Portion = 0,
    Gram,
    Centilitre,
}

impl fmt::Display for Unit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let printable = match *self {
                Unit::Portion => "u.",
                Unit::Gram => "g",
                Unit::Centilitre => "cl",
            };
        write!(f, "{}", printable)
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct Id( u64 );

impl Id {
    fn new() -> Id {
        Id(rand::random())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Id(id) = *self;
        write!(f, "{:X}", id)
    }
}

pub fn add_default_data(data : &mut Data) {

    let other = Section::new("Autre");
    data.add_section(&other);
    let greengrocer = Section::new("Primeur");
    data.add_section(&greengrocer);
    let frozen = Section::new("Surgelé");
    data.add_section(&frozen);
    let dairy_section = Section::new("Crémerie");
    data.add_section(&dairy_section);
    let organic = Section::new("Bio");
    data.add_section(&organic);
    let drink = Section::new("Boisson");
    data.add_section(&drink);
    let starchy = Section::new("Féculents");
    data.add_section(&starchy);
    let condiment = Section::new("Condiment");
    data.add_section(&condiment);
    let butcher = Section::new("Viande");
    data.add_section(&butcher);
    let fishshop = Section::new("Poisson");
    data.add_section(&fishshop);
    
    let vegetable = Group::new("Légume");
    data.add_group(&vegetable);
    let fruit = Group::new("Fruit");
    data.add_group(&fruit);
    let protein = Group::new("Protéine");
    data.add_group(&protein);
    let carbohydrate = Group::new("Féculent");
    data.add_group(&carbohydrate);
    let fat = Group::new("Graisse");
    data.add_group(&fat);
    let dairy_group = Group::new("Laitage");
    data.add_group(&dairy_group);
    let cheese = Group::new("Fromage");
    data.add_group(&cheese);
    
    let tomate = Ingredient::new(
        "tomate fraiche",
        &vegetable,
        &greengrocer,
        &Quantity{val : 1.0, unit : Unit::Portion},
    );
    data.add_ingredient(&tomate);
    
    let pomme = Ingredient::new(
        "pomme",
        &fruit,
        &greengrocer,
        &Quantity{val : 1.0, unit : Unit::Portion},
    );
    data.add_ingredient(&pomme);

    let steak = Ingredient::new(
        "steak haché",
        &protein,
        &butcher,
        &Quantity{val : 1.0, unit : Unit::Portion},
    );
    data.add_ingredient(&steak);
    
    let pain = Ingredient::new(
        "tranche de pain",
        &carbohydrate,
        &other,
        &Quantity{val : 1.0, unit : Unit::Portion},
    );
    data.add_ingredient(&pain);
    
    let huile = Ingredient::new(
        "huile",
        &fat,
        &other,
        &Quantity{val : 5.0, unit : Unit::Centilitre},
    );
    data.add_ingredient(&huile);
    
    let lait = Ingredient::new(
        "lait",
        &dairy_group,
        &dairy_section,
        &Quantity{val : 1.0, unit : Unit::Portion},
    );
    data.add_ingredient(&lait);
    
    let emmental = Ingredient::new(
        "emmental",
        &cheese,
        &dairy_section,
        &Quantity{val : 30.0, unit : Unit::Gram},
    );
    data.add_ingredient(&emmental);
    
}
