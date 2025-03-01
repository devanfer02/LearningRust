struct Person {
    fullname: String,
    gender: bool,
    age: u8
}

struct Rgb (i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u64, 
    height: u64, 
}

impl Rectangle {
    // Associate functions
    // Basically method that can be called only by an instance
    fn area(&self) -> u64 {
        self.width * self.height
    }   
}

impl Rectangle { 
    // Method 
    // Basically static method
    fn square(size: u64) -> Rectangle {
        Rectangle {
            width: size,
            height: size 
        }
    }
}

fn main() {
    let mut person1 = Person {
        fullname: String::from("Devan"),
        gender: true,
        age: 20
    };

    let name = person1.fullname;

    person1.fullname = String::from("Devan Ferrel");

    println!("Old Fullname: {}", name);
    let mut _person2 = build_person("Ferrel", true, 18);
    let mut _person3 = Person{
        .._person2 // like javascript
    };

    let rect = Rectangle{
        width: 50,
        height: 3
    };

    println!("The area of given rectangle is {}", rect.area());
    println!("Rect : {:?}", rect);
    println!("Rect : {:#?}", rect);

    let rect2 = Rectangle::square(30);
    println!("The area of given rectangle is {}", rect2.area());
}

fn build_person(fullname: &str, gender: bool, age: u8) -> Person {
    Person {
        fullname: String::from(fullname),
        gender,
        age,
    }
}