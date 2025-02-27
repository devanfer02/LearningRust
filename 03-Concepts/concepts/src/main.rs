use std::io;

fn main() {
    // Scalar DataTypes:
    /*
        Integer
        Floating Point
        Boolean 
        Character
     */

    let name = "Devan";
    println!("Hello my name is {}", name);

    // Compound DataTypes:
    /*
        Tuples
        Array
     */

    let langs = ("C++", 1.2);
    let used_langs = ["C++", "Go", "Java", "Js"];
    print_langs_used(&used_langs, &langs);
    println!("Sum of 4 + 3 is {}", add_num(3, 5));

    let result = if add_num(4, 7) == 11 { "Correct" } else { "Wrong" };
    println!("Result: {}",result);

    //shadowing
    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt;
        }
    };
    println!("Result: {}",result);

    for num in 1..=4 {
        println!("{}", num);
    }
}

fn print_langs_used(used_langs: &[&str], pro_lang: &(&str, f32)) {
    println!("I have used {} for {} years", pro_lang.0, pro_lang.1);
    println!("Other languages i have also used:");
    used_langs.iter().enumerate().for_each(|(idx, lang)| {
        println!("{}. {}", idx + 1, lang);
    });
}

fn add_num(x: i32, y: i32) -> i64 {
    let sum: i64 = (x + y).into();
    return sum;
}