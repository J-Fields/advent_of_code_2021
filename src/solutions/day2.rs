use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map,
    sequence::separated_pair,
    Finish, IResult,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Solution;

enum Move {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Move {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                alt((tag("forward"), tag("down"), tag("up"))),
                multispace1,
                map(digit1, |x: &str| x.parse::<u32>().unwrap()),
            ),
            |(dir, distance)| {
                if dir == "forward" {
                    Move::Forward(distance)
                } else if dir == "down" {
                    Move::Down(distance)
                } else {
                    Move::Up(distance)
                }
            },
        )(input)
    }
}

trait Position {
    fn transform(&self, mov: Move) -> Self;
}

#[derive(Default)]
struct Part1Position {
    pub horizontal: u32,
    pub depth: u32,
}

impl Position for Part1Position {
    fn transform(&self, mov: Move) -> Self {
        match mov {
            Move::Forward(distance) => Self {
                horizontal: self.horizontal + distance,
                depth: self.depth,
            },
            Move::Down(distance) => Self {
                horizontal: self.horizontal,
                depth: self.depth + distance,
            },
            Move::Up(distance) => Self {
                horizontal: self.horizontal,
                depth: self.depth - distance,
            },
        }
    }
}

#[derive(Default)]
struct Part2Position {
    pub horizontal: u32,
    pub depth: u32,
    aim: u32,
}

impl Position for Part2Position {
    fn transform(&self, mov: Move) -> Self {
        match mov {
            Move::Forward(distance) => Self {
                horizontal: self.horizontal + distance,
                depth: self.depth + distance * self.aim,
                aim: self.aim,
            },
            Move::Down(units) => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + units,
            },
            Move::Up(units) => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - units,
            }
        }
    }
}

pub struct Day2 {}

impl Solution for Day2 {
    fn part1(&self, input: BufReader<File>) -> u32 {
        let final_pos = input
            .lines()
            .map(|line| Move::parse(line.unwrap().as_str()).finish().unwrap().1)
            .fold(Part1Position::default(), |pos, mov| pos.transform(mov));
        final_pos.horizontal * final_pos.depth
    }

    fn part2(&self, input: BufReader<File>) -> u32 {
        let final_pos = input
            .lines()
            .map(|line| Move::parse(line.unwrap().as_str()).finish().unwrap().1)
            .fold(Part2Position::default(), |pos, mov| pos.transform(mov));
        final_pos.horizontal * final_pos.depth
    }
}
