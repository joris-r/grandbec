extern crate gtk;
use gtk::prelude::*;

use std::rc::*;
use std::cell::*;

use logic::*;
use gui_unit::*;

pub struct GuiGoods {
    data : Rc<RefCell<Data>>,
    grid : gtk::Grid,
    last : i32,
    myself : Option<Rc<RefCell<GuiGoods>>>,
}

impl GuiGoods {
    pub fn new(data : &Rc<RefCell<Data>>) -> GuiGoods {
        let gs = GuiGoods {
            data : data.clone(),
            grid : gtk::Grid::new(),
            last : -1,
            myself : None,
        };
        gs
    }
    
    pub fn set_myself(&mut self, myself : &Rc<RefCell<GuiGoods>>) {
        self.myself = Some(myself.clone());
    }
    
    pub fn get_main_widget(&self) -> gtk::Widget {
        self.grid.clone().upcast::<gtk::Widget>()
    }
    
    pub fn setup(&mut self){
        self.show();
    }
    
    fn show(&mut self) {
        for w in self.grid.get_children() {
            w.destroy();
        }
        
        self.grid.set_row_spacing(5);
        self.grid.set_column_spacing(10);
        
        let i_name = gtk::Label::new("");
        i_name.set_markup("<b>Ingrédients</b>");
        i_name.set_halign(gtk::Align::Start);
        self.last += 1;
        self.grid.attach(&i_name, 1, self.last, 1, 1);
        
        let i_qval = gtk::Label::new("");
        i_qval.set_markup("<b>Par défaut</b>");
        i_qval.set_halign(gtk::Align::Start);
        self.grid.attach(&i_qval, 2, self.last, 1, 1);
        
        let i_qunit = gtk::Label::new("");
        i_qunit.set_markup("<b>Unité</b>");
        i_qunit.set_halign(gtk::Align::Start);
        self.grid.attach(&i_qunit, 3, self.last, 1, 1);
        
        let i_group_name = gtk::Label::new("");
        i_group_name.set_markup("<b>Groupe alimentaire</b>");
        i_group_name.set_halign(gtk::Align::Start);
        self.grid.attach(&i_group_name, 4, self.last, 1, 1);
        
        let i_section_name = gtk::Label::new("");
        i_section_name.set_markup("<b>Rayon de magasin</b>");
        i_section_name.set_halign(gtk::Align::Start);
        self.grid.attach(&i_section_name, 5, self.last, 1, 1);

        // all the ingredients
        for i in self.data.borrow().iter_ingredients() {
            self.last += 1;
            self.show_article(i, self.last);
        }
        
        // Creation of an ingredient
        let add_but = gtk::Button::new_with_label("Ajouter");
        self.last += 1;
        self.grid.attach(&add_but, 0, self.last, 6, 1);
        
        let data_clone = self.data.clone();
        let myself = self.myself.clone().unwrap();
        add_but.connect_clicked(move |_| {
            let i = data_clone.borrow_mut().new_ingredient("Nouvel ingrédient").clone();
            let g = myself.borrow().grid.clone();
            g.insert_row(myself.borrow().last);
            myself.borrow().show_article(&i, myself.borrow().last);
            myself.borrow_mut().last += 1;
            g.show_all();
        });
        
        self.grid.show_all();
    }
        
    fn show_article(&self, i : & Ingredient, line : i32) {
        
        // Name
        let i_name_eb = gtk::EntryBuffer::new(Some(&i.name as &str));
        let i_name = gtk::Entry::new_with_buffer(&i_name_eb);
        i_name.set_halign(gtk::Align::Start);
        self.grid.attach(&i_name, 1, line, 1, 1);
        
        let data_clone = self.data.clone();
        let i_name_eb_clone = i_name_eb.clone();
        let i_id = i.id;
        i_name.connect_changed(move |_| {
            let mut d = data_clone.borrow_mut();
            let mut i = d.get_ingredient_mut(i_id);
            i.name = i_name_eb_clone.get_text();
        });

        
        // Quantity value
        let i_qval = gtk::SpinButton::new_with_range(
            1.0, 1000.0, 1.0);
        i_qval.set_halign(gtk::Align::Start);
        self.grid.attach(&i_qval, 2, line, 1, 1);
        i_qval.set_digits(1);
        i_qval.set_value((*i).quantity.val);
        
        // store quantity change
        let data_clone = self.data.clone();
        let i_id = i.id;
        i_qval.connect_value_changed(move |w| {
            let mut d = data_clone.borrow_mut();
            let mut i = d.get_ingredient_mut(i_id);
            i.quantity.val = w.get_value();
        });
        
        // Quantity unit
        let i_qunit = create_combo_of_unit(&i.quantity.unit);
        i_qunit.set_halign(gtk::Align::Start);
        self.grid.attach(&i_qunit, 3, line, 1, 1);
        
        // store unit change
        let data_clone = self.data.clone();
        let i_id = i.id;
        i_qunit.connect_changed(move |w| {
            let mut d = data_clone.borrow_mut();
            let mut i = d.get_ingredient_mut(i_id);
            i.quantity.unit = read_unit(&w);
        });
                
        // Food group
        let i_group = gtk::ComboBoxText::new();
        i_group.set_halign(gtk::Align::Start);
        self.grid.attach(&i_group, 4, line, 1, 1);
        
        for x in self.data.borrow().iter_groups() {
            i_group.insert(
                -1,
                &x.id.to_string() as &str,
                &x.name.to_string()  as &str);
        }
        i_group.set_active_id(&i.group.id.to_string() as &str);

        // Food group change
        let data_clone = self.data.clone();
        let i_id = i.id;
        i_group.connect_changed(move |w| {
            let g_id = w.get_active_id().unwrap();
            let g_id = Id(u64::from_str_radix(&g_id, 16).unwrap());
            let mut data = data_clone.borrow_mut();
            let g : Group = data.get_group(g_id).unwrap().clone();
            let mut i = data.get_ingredient_mut(i_id);
            i.group = g;
        });
        
        
        // Grocery section
        let i_section = gtk::ComboBoxText::new();
        i_section.set_halign(gtk::Align::Start);
        self.grid.attach(&i_section, 5, line, 1, 1);
        
        for x in self.data.borrow().iter_sections() {
            i_section.insert(
                -1,
                &x.id.to_string() as &str,
                &x.name.to_string()  as &str);
        }
        i_section.set_active_id(&i.section.id.to_string() as &str);

        // Grocery section change
        let data_clone = self.data.clone();
        let i_id = i.id;
        i_section.connect_changed(move |w| {
            let s_id = w.get_active_id().unwrap();
            let s_id = Id(u64::from_str_radix(&s_id, 16).unwrap());
            let mut data = data_clone.borrow_mut();
            let s : Section = data.get_section(s_id).unwrap().clone();
            let mut i = data.get_ingredient_mut(i_id);
            i.section = s;
        });
        
        // Deletion of ingredient
        let rm_but = gtk::Button::new_with_label("X");
        self.grid.attach(&rm_but, 6, line, 1, 1);
        
        let data_clone = self.data.clone();
        let i_id = i.id;
        let myself = self.myself.clone().unwrap();
        rm_but.connect_clicked(move |w| {
            if ! data_clone.borrow().is_ingredient_used(i_id) {
                {
                    let mut data = data_clone.borrow_mut();
                    data.remove_ingredient(i_id);
                }
                myself.borrow_mut().show();
            } else {
                let md = gtk::MessageDialog::new(
                    Some(&w.get_toplevel().unwrap().downcast::<gtk::Window>().unwrap()),
                    gtk::DIALOG_MODAL,
                    gtk::MessageType::Warning,
                    gtk::ButtonsType::Ok,
                    "Suppression impossible : l'ingrédient est utilisé."
                );
                md.connect_response(|w,_| {
                    w.destroy();
                });
                md.show();
            }
        });
    }
    
}
