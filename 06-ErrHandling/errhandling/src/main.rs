use std::{fs::File, io::Read};
use std::io::{self, ErrorKind, Write};

const CONTENT: &str = 
"package main

func main() {
    println(\"Hello, World!\")
}
";

fn main() {
    let file_name = "chello.txt";
    let res = File::open(file_name);

    let mut file = match res {
        Ok(file) => file, 
        Err(error) => 
            match error.kind() {
                ErrorKind::NotFound => match File::create(file_name) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tidak dapat membuat file: {:?}", e),
                },
                other_error => panic!("Tidak dapat membuka file: {:?}", other_error),        
            }
        
    };

    let mut content  = String::new();
    file.read_to_string(&mut content).unwrap();

    println!("{:?}", content);

    File::create("main.go").expect("Failed to create file");

    write_content_to_file("main.go".to_string(), CONTENT.to_string());

    let content = read_content_from_file("main.go".to_string()).expect("Failed to read file");
    println!("{:?}", content);
}

fn read_content_from_file(file_name: String) -> Result<String, io::Error> {
    let mut content  = String::new();
    File::open(file_name)?.read_to_string(&mut content)?; 

    Ok(content)
    
}

fn write_content_to_file(file_name: String, content: String) -> Result<(), io::Error> {
    File::create(file_name)?.write_all(content.as_bytes())?;

    Ok(())
}

