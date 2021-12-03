extern crate nom;

use std::num::ParseIntError;
use nom::bytes::complete::take_while1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::{IResult, Parser};
use nom::error::ParseError;
use nom::multi::separated_list0;

fn str_to_i32(input: &str) -> Result<i32, ParseIntError> {
    i32::from_str_radix(input, 10)
}

fn is_digit(input: char) -> bool {
    input.is_digit(10)
}

pub fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(take_while1(is_digit), str_to_i32)(input)
}

pub fn parse_lines<'a, F, O, E>(
    parser: F
) -> impl Parser<&'a str, Vec<O>, E>
    where
        F: Parser<&'a str, O, E> + Copy,
        E: ParseError<&'a str>
{
    move |i: &'a str| {
        separated_list0(line_ending, parser)(i)
    }
}