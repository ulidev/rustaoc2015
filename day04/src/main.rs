use md5;

fn valid2(input : &String) -> bool{
    return input.chars().nth(0).unwrap() == '0' && input.chars().nth(1).unwrap() == '0' && input.chars().nth(2).unwrap() == '0' && input.chars().nth(3).unwrap() == '0'
    && input.chars().nth(4).unwrap() == '0' && input.chars().nth(5).unwrap() == '0'
}

fn valid1(input : &String) -> bool{
    return input.chars().nth(0).unwrap() == '0' && input.chars().nth(1).unwrap() == '0' && input.chars().nth(2).unwrap() == '0' && input.chars().nth(3).unwrap() == '0'
    && input.chars().nth(4).unwrap() == '0'
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut i1 :i32= 0;

    let mut current_value = format!("{:x}",md5::compute(format!("{}{}", data, i1)));

    while !valid1(&current_value) {
        i1 += 1;
        current_value = format!("{:x}",md5::compute(format!("{}{}", data, i1)));
    }

    let mut i2 :i32 = i1;
    current_value = format!("{:x}",md5::compute(format!("{}{}", data, i2)));
    
    while !valid2(&current_value) {
        i2 += 1;
        current_value = format!("{:x}",md5::compute(format!("{}{}", data, i2)));
    }

    println!(
        "Part 1: {}",
        i1
    );

    println!(
        "Part 2: {}",
        i2
    );
}