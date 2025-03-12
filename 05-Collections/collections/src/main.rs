// Collections are allocated in Heap unlike Array or Tuple
use std::collections::HashMap;
fn main() {
    // Vector is dynamic array
    let mut _v: Vec<i32> = Vec::new();
    _v.push(1);
    _v.push(2);
    _v.push(3);
    
    let first = _v[0];
    _v.push(30);
    println!("The first element is {}", first);

    // Can declare vector like this also, rust will automatically infer type value
    let mut _v2 = vec![1, 3, 3];

    let second = _v[1];
    println!("The second element is {}", second);

    // Handle IndexOutOfBond gracefully
    match _v2.get(3) {
        Some(forth) => println!("The forth element is {}", forth),
        None => println!("There is no forth element")
    }

    // Looping Vector
    for num in &mut _v {
        *num *= 2;
        println!("{}", num);
    }

    // String in Rust
    let mut name = String::from("Deva");
    let last_name = String::from("Ferrel");
    name.push_str("n");
    println!("Name: {}", name);

    // Format macro doesnt move the ownership
    let fullname = format!("{} {}", name, last_name);
    println!("Fullname: {}", fullname);

    // HashMaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Red"), 50);

    for team in &scores {
        println!("Team Name: {}, Team Score: {}", team.0, team.1);
    }

    print_val(String::from("Blue"), &scores);
    print_val(String::from("Yellow"), &scores);
    
}

fn print_val(key: String, map: &HashMap<String, i32>) {
    match map.get(&key) {
        Some(val) => println!("Value is : {}", val),
        None => println!("There is no key")
    }
}
