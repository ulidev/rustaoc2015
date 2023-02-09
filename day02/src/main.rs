struct Present{
    length : i32,
    width : i32,
    height : i32
}

impl Present{
    fn new(l : i32, w : i32, h : i32) -> Present{
        Present{
            length : l,
            width : w,
            height : h,
        }
    }

    fn get_needed_paper(&self) -> i32{
        3 * self.length * self.width + 2 * self.width * self.height + 2 * self.length * self.height
    }

    fn get_needed_ribbon(&self) -> i32{
        2 * self.length + 2 * self.width + self.length * self.width * self.height
    }
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut paper_counter: i32 = 0;

    let mut ribbon_counter : i32 = 0;

    for line in data.lines() {
        let mut dimensions : Vec<i32> = line.split('x').map(|d| d.parse().unwrap()).collect();
        dimensions.sort();
        let current_present: Present = Present::new(dimensions[0], dimensions[1], dimensions[2]);

        paper_counter += current_present.get_needed_paper();

        ribbon_counter += current_present.get_needed_ribbon();

    }
    

    println!(
        "Part 1: {}",
        paper_counter
    );

    println!(
        "Part 2: {}",
        ribbon_counter
    );
}