// By Default, Rust use private modifier for module, struct and fields

use std::mem;

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String 
}

// Except for enum variants, they are public by default
pub enum Appetizer {
    Soup, 
    Salad 
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches")
        }
    }
}

pub struct Menu {
    pub name: String,
    price: u32 
}

impl Menu {
    pub fn new(name: &str, price: u32) -> Menu {
        Menu {
            name: String::from(name),
            price
        }
    }
}

pub struct Order {
    pub menus: Vec<Menu>,
    total_price: u32 
}

impl Order {
    pub fn add_menu(&mut self, menu: Menu ) {
        self.menus.push( menu );
    }

    pub fn new() -> Order {
        Order {
            menus: vec![],
            total_price: 0
        }
    }

    pub fn calculate_total(&mut self) {
        let total: u32 = self.menus
            .iter()
            .map(|menu| menu.price)
            .sum();

        self.total_price = total; 
    }
}