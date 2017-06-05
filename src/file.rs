extern crate serde;
extern crate serde_json;

use serde_json::value::Value;

use std::path::Path;
use std::collections::HashMap;

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
            let ingredients = recipe.ingredients.values()
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

pub fn load_all(data : &mut Data, path : &Path) {
    use std::fs::File;
    use serde_json::value::from_value;
    {
        let file = File::open(path.join(CATALOG_FILE_NAME)).unwrap();
        let json: Value = serde_json::from_reader(file).unwrap();
        let json = json.as_object().unwrap();
        
        let sections = json
            .get("sections").unwrap()
            .as_array().unwrap();
        let groups = json
            .get("groups").unwrap()
            .as_array().unwrap();
        let ingredients = json
            .get("ingredients").unwrap()
            .as_array().unwrap();
        
        for section in sections {
            let id = deserialize_id(&section["id"]);
            let name = section["name"].as_str().unwrap();
            // TODO convertion own/ref pas performante et pas logique
            data.add_section(&Section{id:id, name:name.to_string()});
        }
        
        for group in groups {
            let id = deserialize_id(&group["id"]);
            let name = group["name"].as_str().unwrap();
            // TODO convertion own/ref pas performante et pas logique
            data.add_group(&Group{id:id, name:name.to_string()});
        }
        
        for ingredient in ingredients {
            let id = deserialize_id(&ingredient["id"]);
            let name = ingredient["name"].as_str().unwrap();
            let group_id = deserialize_id(&ingredient["group"]);
            let group = data.get_group(group_id).unwrap().clone();
            let section_id = deserialize_id(&ingredient["section"]);
            let section = data.get_section(section_id).unwrap().clone();
            let quantity = ingredient["quantity"].clone();
            let quantity : Quantity = from_value(quantity).unwrap();
            data.add_ingredient(&Ingredient{
                id : id,
                name : name.to_string(),
                group : group,
                section : section,
                quantity : quantity,
            });
        }
    }
    {
        let file = File::open(path.join(BOOK_FILE_NAME)).unwrap();
        let json: Value = serde_json::from_reader(file).unwrap();
        let json = json.as_object().unwrap();
        
        let recipes = json
            .get("recipes").unwrap()
            .as_array().unwrap();
        for recipe in recipes {
            let id = deserialize_id(&recipe["id"]);
            let name = recipe["name"].as_str().unwrap();
            let note = recipe["note"].as_str().unwrap();
            let ingredients = recipe
                .get("ingredients").unwrap()
                .as_array().unwrap();
            let mut r = Recipe {
                id : id,
                name : name.to_string(),
                note : note.to_string(),
                ingredients : HashMap::new(),
            };
            for ingredient in ingredients {
                let id = deserialize_id(&ingredient["id"]);
                let quantity = ingredient["quantity"].clone();
                let quantity : Quantity = from_value(quantity).unwrap();
                let mut i = data.get_ingredient(id).unwrap().clone();
                i.quantity = quantity;
                r.ingredients.insert(i.id,i);
            }
            data.add_recipe(&r);
        }
    }
}

fn deserialize_id(json : &Value) -> Id {
    let id = json.as_str().unwrap();
    let id = u64::from_str_radix(id, 16).unwrap();
    Id(id)
}
