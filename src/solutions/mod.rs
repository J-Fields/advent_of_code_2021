mod day1;
mod day2;

use std::{fs::File, io::BufReader};


pub fn get_solution(day: usize) -> Option<Box<dyn Solution>> {
    if day == 1 {
        Some(Box::new(day1::Day1 {}))
    } else if day == 2 {
        Some(Box::new(day2::Day2 {}))
    } else {
        None
    }
}

pub trait Solution {
    fn part1(&self, input: BufReader<File>) -> u32;
    fn part2(&self, input: BufReader<File>) -> u32;
}
