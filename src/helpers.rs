extern crate nom;

use nom::bytes::complete::take_while1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::{IResult, Parser};
use nom::error::ParseError;
use nom::multi::separated_list0;

fn parse_number(input: &str, radix: u32) -> IResult<&str, i32> {
    map_res(take_while1(|input: char| input.is_digit(radix)), |str| i32::from_str_radix(str, radix))(input)
}

pub fn parse_decimal_number(input: &str) -> IResult<&str, i32> {
    parse_number(input, 10)
}

pub fn parse_binary_number(input: &str) -> IResult<&str, i32> {
    parse_number(input, 2)
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