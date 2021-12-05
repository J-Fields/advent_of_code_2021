mod day1;

use std::{fs::File, io::BufReader};

use self::day1::Day1;

pub fn get_solution(day: usize) -> Option<Box<dyn Solution>> {
    if day == 1 {
        Some(Box::new(Day1 {}))
    } else {
        None
    }
}

pub trait Solution {
    fn part1(&self, input: BufReader<File>) -> u32;
    fn part2(&self, input: BufReader<File>) -> u32;
}
