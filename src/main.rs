mod solutions;

use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::solutions::get_solution;

#[derive(clap::Parser)]
#[clap()]
struct Opts {
    day: usize,
    part: u32,
}

fn main() {
    let opts = Opts::parse();

    // Create a path to the desired file
    let input_path_str = format!("data/day{}.txt", opts.day);
    let input_path = Path::new(&input_path_str);
    let input_display = input_path.display();

    let input = match File::open(&input_path) {
        Err(why) => panic!("Couldn't open {}: {}", input_display, why),
        Ok(file) => BufReader::new(file),
    };

    let solution = get_solution(opts.day).unwrap();

    if opts.part == 1 {
        println!("Result: {}", solution.part1(input));
    } else if opts.part == 2 {
        println!("Result: {}", solution.part2(input));
    } else {
        panic!("Invalid part!")
    }
}
