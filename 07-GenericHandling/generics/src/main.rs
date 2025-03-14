struct Point<T> {
    x: T,
    y: T,
}

// x method is available for all types of Point
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x 
    }
}

// y method is available only for Point with Type f64
impl Point<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

struct Pair<K, V> {
    key: K,
    value: V 
}

fn main() {
    let _p1 = Point { x: 5, y: 10 };
    let _p2 = Point { x: 1.0, y: 4.0 };

    // _p1.y(); // error

    let _pair1 = Pair { key: "hola", value: 1 };

    let char_list = vec!['y', 'm', 'a', 'q'];
    let num_list = vec![34, 50, 25, 100, 65];

    let largest_char = get_largest(&char_list);
    let largest_num = get_largest(&num_list);

    println!("Num: {}", largest_num);
    println!("Char: {}", largest_char);
}

// T must implement PartialOrd and Copy
fn get_largest<T: PartialOrd + Copy>(num_list: &Vec<T>) -> T {;
    let mut largest = num_list[0];

    for number in num_list {
        // Dereferencing the number first 
        if *number > largest { 
            largest = *number;
        }
    }

    largest 
}