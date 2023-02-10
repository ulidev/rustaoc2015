use std::collections::HashSet;

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut visited_houses1 : HashSet<(i32, i32)> = HashSet::new();
    let mut visited_houses2 : HashSet<(i32, i32)> = HashSet::new();

    let mut position: (i32, i32) = (0,0);
    let mut santa: (i32, i32) = (0,0);
    let mut robo: (i32, i32) = (0,0);

    visited_houses1.insert((0,0));
    visited_houses2.insert((0,0));


    for (i, direction) in data.chars().enumerate(){
        match direction {
            '^' => {
                match i%2{
                    0 => santa.1 += 1,
                    _ => robo.1 += 1
                }
                position.1 += 1;
            }
            'v' => {
                match i%2{
                    0 => santa.1 -= 1,
                    _ => robo.1 -= 1
                }
                position.1 -= 1;
            }
            '>' => {
                match i%2{
                    0 => santa.0 += 1,
                    _ => robo.0 += 1
                }
                position.0 += 1;
            }
            '<' => {
                match i%2{
                    0 => santa.0 -= 1,
                    _ => robo.0 -= 1
                }
                position.0 -= 1;
            }
            _ => println!("What is this? {}", direction)
        };
        match i%2 {
            0 => visited_houses2.insert((santa.0, santa.1)),
            _ => visited_houses2.insert((robo.0, robo.1))
        };
        visited_houses1.insert((position.0, position.1));
    }

    println!(
        "Part 1: {}",
        visited_houses1.len()
    );

    println!(
        "Part 2: {}",
        visited_houses2.len()
    );
}