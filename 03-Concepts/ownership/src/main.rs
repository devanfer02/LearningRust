/*
# Rust Ownership

Ownership allows rust to make memory safety guarantees without the use of garbage collector
Ownership model in rust is a model to manage memory, it has faster runtime, smaller program size, more control over memory but slower development and high learning curve other than GC and Manual.

### Ownership Rules

1. Each value in rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped 

### References Rules
1. At any given time, you can have either one mutable reference or any number of immutable references
2. References must be always valid

 */

fn main() {
    {
        let _s = String::from("Hello"); // s is only valid from this point and this scope
    }

    let x = 5;
    let _y = x; // Copy, x is allocated in stack

    let s0 = String::from("Hello");
    // let s2 = s0; // This one will move because s0 is allocated in heap
    let s2 = s0.clone(); // This one will copy
    println!("{}", s2);
    
    let s2 = give_owship();
    let s3 = String::from("Holla");
    let s4 = take_and_give(s3);
    println!("s2: {} & s4: {}", s2, s4);

    let mut s4 = take_and_give(s4);

    change(&mut s4, "Devan");
    println!("{}", s4);

    take_owship("Hello World!!".to_string());

    // Its OK to have multiple immuteable references
    // But its not OK tu have multiple mutable references
    // To prevent data races since Immutebale is read only while Mutable can read and write
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

        
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // will result error, can't borrow s as mutable more than once

    // String slices
    let mut s = String::from("Hello world");
    let h0 = &s[..5]; // Hello
    let w0 = &s[6..]; // World

    let s2 = "Hello world"; // String literal, stored in stack

    let w0 = first_word(&s);
    println!("{}", w0);
}


// This function will take the ownership if you pass the param as variable
// Since the variable is allocated in heap
// So the variable can't be used again after passing it to this function
// We need to pass it as reference like fn take_owship(some_str: &String)
// Passing reference called borrowing and it does not move ownership
fn take_owship(some_str: String) {
    println!("{}", some_str);
}

fn give_owship() -> String {
    let some_val = String::from("Hai");

    some_val // you can directly return variable without return keyword
}

fn take_and_give(s: String) -> String {
    s
}

// This function accept mutable reference since the default is imutable
fn change(s: &mut String, name: &str) {
    s.clear();
    s.push_str(&format!("Hello, {}", name));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // This one count as return

    // &s[..]; // This one doesnt??? wtf
}