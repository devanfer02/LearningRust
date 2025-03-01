use moduleproj::restaurant::{Menu, Order, Breakfast};

fn main() {
    let mut my_breakfast = Breakfast::summer("Bread");
    my_breakfast.toast = String::from("Egg"); 

    let mut order = Order::new();
    order.add_menu(Menu::new("Kebab", 18_000));

    println!("Order 1 Name: {}", order.menus[0].name);
}