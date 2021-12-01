extern crate nom;

use std::io::Error;
use crate::helpers::{parse_lines, parse_number};

const INPUT_DAY01: &str = include_str!("inputs/day01.txt");


fn parse_input_day01(input: &str) -> Vec<i32> {
    parse_lines(parse_number)(input).unwrap().1
}

pub fn day1() -> Result<(), Error> {
    println!("part 2: {:?}", parse_input_day01(INPUT_DAY01));

    Ok(())
}

