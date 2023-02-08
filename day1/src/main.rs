use std::fs::File;
use std::io::prelude::*;

fn first(data : &String) -> i32{
    let mut counter : i32 = 0;

    for c in data.chars(){
        if c == '(' {
            counter += 1;
        } else if c == ')' {
            counter -= 1;
        }
    }
    counter
}

fn second(data : &String) -> i32{
    let mut counter : i32 = 0;

    for (i, c) in data.chars().enumerate(){
        if c == '('{
            counter += 1;
        } else if c == ')'{
            counter -= 1;
            if counter < 0{
                return (i+1).try_into().unwrap()
            }
        }
    }

    0
}

fn main() {
    let mut file = File::open("input")
        .expect("File not found");
    let mut data : String = String::new();
    file.read_to_string(&mut data)
        .expect("Parsed incorrectly");


    println!("First day solution: {}", first(&data));
    println!("Second day solution: {}", second(&data));
}
