
#![allow(dead_code)]


extern crate rand;

use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map;

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
            book : Book {
                recipes : HashMap::new(),
            },
            planning : Planning {},
            history : History {},
            stock : Stock {},
        }
    }
    
    // TODO faire un move plutot ? (et renvoyer une référece ?)
    pub fn add_section(&mut self, section : &Section) {
        // TODO check if id is unique
        self.catalog.sections.insert(section.id, section.clone());
    }
    
    pub fn get_section(&self, id : Id) -> Option<&Section> {
        self.catalog.sections.get(&id)
    }
    
    pub fn iter_sections(&self) -> hash_map::Values<Id, Section> {
        self.catalog.sections.values()
    }
    
    pub fn add_group(&mut self, group : &Group) {
        // TODO check if id is unique
        self.catalog.groups.insert(group.id, group.clone());
    }
    
    pub fn get_group(&self, id : Id) -> Option<&Group> {
        self.catalog.groups.get(&id)
    }
    
    pub fn iter_groups(&self) -> hash_map::Values<Id, Group> {
        self.catalog.groups.values()
    }
    
    pub fn add_ingredient(&mut self, ingredient : &Ingredient) {
        // TODO check if id is unique
        self.catalog.ingredients.insert(ingredient.id, ingredient.clone());
    }
    
    pub fn get_ingredient(&self, id : Id) -> Option<&Ingredient> {
        self.catalog.ingredients.get(&id)
    }
    
    pub fn iter_ingredients(&self) -> hash_map::Values<Id, Ingredient> {
        self.catalog.ingredients.values()
    }
        
    pub fn add_recipe(&mut self, recipe : &Recipe) {
        // TODO check if id is unique
        self.book.recipes.insert(recipe.id, recipe.clone());
    }
        
    pub fn get_recipe(&self, id : Id) -> Option<&Recipe> {
        self.book.recipes.get(&id)
    }
    
    pub fn iter_recipes(&self) -> hash_map::Values<Id, Recipe> {
        self.book.recipes.values()
    }
    
} // impl Data

        
struct Catalog {
    ingredients : HashMap<Id,Ingredient>,
    sections : HashMap<Id,Section>,
    groups : HashMap<Id,Group>,
}


struct Book {
    recipes : HashMap<Id,Recipe>,
}

struct Planning {
}

struct History {
}

struct Stock {
}

#[derive(Clone)]
pub struct Recipe {
    pub id : Id,
    pub name : String,
    pub note : String,
    pub ingredients : Vec<Ingredient>,
}

impl Recipe {
    pub fn new(name : &str, note : &str) -> Recipe {
        Recipe {
            id : Id::new(),
            name : name.to_string(),
            note : note.to_string(),
            ingredients : vec![],
        }
    }
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
    pub id : Id,
    pub name : String,
}

impl Group {
    pub fn new(name : &str) -> Group {
        Group{
            id : Id::new(),
            name : name.to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Section {
    pub id : Id,
    pub name : String,
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

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    val : f64,
    unit : Unit,
}


#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Unit {
    #[serde(rename = "p")]
    Portion = 0,
    #[serde(rename = "g")]
    Gram,
    #[serde(rename = "cl")]
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
pub struct Id( pub u64 );

impl Id {
    fn new() -> Id {
        Id(rand::random())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Id(id) = *self;
        write!(f, "{:016X}", id)
    }
}
