use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Solution;

pub struct Day1 {}

impl Solution for Day1 {
    fn part1(&self, input: BufReader<File>) -> u32 {
        input
            .lines()
            .map(|line| line.unwrap().parse::<u32>().unwrap())
            .tuple_windows::<(_, _)>()
            .filter(|(x, y)| y > x)
            .count()
            .try_into()
            .unwrap()
    }

    fn part2(&self, input: BufReader<File>) -> u32 {
        input
            .lines()
            .map(|line| line.unwrap().parse::<u32>().unwrap())
            .tuple_windows::<(_, _, _)>()
            .map(|(x, y, z)| x + y + z)
            .tuple_windows::<(_, _)>()
            .filter(|(x, y)| y > x)
            .count()
            .try_into()
            .unwrap()
    }
}
