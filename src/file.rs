extern crate serde;
extern crate serde_json;

use serde_json::value::Value;

use std::path::Path;

use logic::*;

const CATALOG_FILE_NAME : &str = "catalog.json";
const BOOK_FILE_NAME : &str = "recipes_book.json";

pub fn save_all(data : &Data, path : &Path) {
    use std::io::prelude::*;
    use std::fs::File;
    {
        let mut sections = vec![];
        for ref section in data.iter_sections() {
            let json = json!({
                "id": section.id.to_string(),
                "name": section.name,
            });
            sections.push(json);
        }
        let sections = Value::Array(sections);
        
        let mut groups = vec![];
        for ref group in data.iter_groups() {
            let json = json!({
                "id": group.id.to_string(),
                "name": group.name,
            });
            groups.push(json);
        }
        let groups = Value::Array(groups);

        let ingredients = data.iter_ingredients().map(serialize_ingredient);
        let ingredients = Value::Array(ingredients.collect());
        
        let catalog = json!({
            "sections": sections,
            "groups": groups,
            "ingredients": ingredients,
        });
    
        let mut file = File::create(path.join(CATALOG_FILE_NAME)).unwrap();
        writeln!(file, "{}", catalog).unwrap();
    }
    {
        let mut recipes = vec![];
        for ref recipe in data.iter_recipes() {
            let ingredients = recipe.ingredients.iter()
                .map(serialize_ingredient_use)
                .collect();
            let json = json!({
                "id" : recipe.id.to_string(),
                "name" : recipe.name,
                "note" : recipe.note,
                "ingredients" : Value::Array(ingredients),
            });
            recipes.push(json);
        }
        let recipes = Value::Array(recipes);
        
        let book = json!({
            "recipes": recipes,
        });
        let mut file = File::create(path.join(BOOK_FILE_NAME)).unwrap();
        writeln!(file, "{}", book).unwrap();
    }
    
}

fn serialize_ingredient(ingredient : & Ingredient) -> Value {
    json!({
        "id" : ingredient.id.to_string(),
        "name" : ingredient.name,
        "group" : ingredient.group.id.to_string(),
        "section" : ingredient.section.id.to_string(),
        "quantity" : ingredient.quantity,
    })
}

fn serialize_ingredient_use(ingredient : & Ingredient) -> Value {
    json!({
        "id" : ingredient.id.to_string(),
        "quantity" : ingredient.quantity,
    })
}
