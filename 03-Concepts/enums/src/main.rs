use core::fmt;

#[derive(Debug)]
enum Role {
    USER,
    ADMIN
}

enum IpAddrKind {
    V4(String),
    V6(String)
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Role::USER => "User",
            Role::ADMIN => "Admin"
        };

        writeln!(f, "{}", text)
    }
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    role: Role
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Username: {}, Role: {}", self.username, self.role)
    }
}

fn main() {
    let user1 = User {
        username: String::from("dvnnfrr"),
        password: String::from("123"),
        role: Role::USER
    };

    println!("Debugging User: {:?}", user1);
    println!("Printing User: {}", user1);

    let localhost = IpAddrKind::V4(String::from("127.0.0.1"));

    let x = 5;
    let y = Some(5);
    let z: Option<i32> = None; 

    let sum = x + y.unwrap_or(0);

    println!("Sum of x + y : {}", sum);
    println!("Sum of y + z : {}", sum_of_xy(y, z));

    let z = plus_one(y);
    println!("Result Z : {}", z.unwrap_or(-1));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val + 1)
    }
}

fn sum_of_xy(x: Option<i32>, y: Option<i32>) -> i32 {
    let tempX = match x {
        None => 0,
        Some(val) => val 
    };

    let tempY = match y {
        None => 0,
        Some(val) => val 
    };

    tempX + tempY 
}