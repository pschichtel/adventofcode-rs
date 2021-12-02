use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::{IResult, Parser};
use nom::sequence::preceded;
use crate::helpers::{parse_lines, parse_number};
use crate::year2021::day02::Instr::{Down, Forward, Up};

const INPUT: &str = include_str!("inputs/day02.txt");
const EXAMPLE_INPUT: &str = include_str!("inputs/day02_example.txt");

#[derive(Debug)]
enum Instr {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_forward(input: &str) -> IResult<&str, Instr> {
    preceded(tag("forward "), parse_number).map(|n| { Instr::Forward(n) }).parse(input)
}

fn parse_up(input: &str) -> IResult<&str, Instr> {
    preceded(tag("up "), parse_number).map(|n| { Instr::Up(n) }).parse(input)
}

fn parse_down(input: &str) -> IResult<&str, Instr> {
    preceded(tag("down "), parse_number).map(|n| { Instr::Down(n) }).parse(input)
}

fn parse_instr(input: &str) -> IResult<&str, Instr> {
    alt((parse_forward, parse_up, parse_down))(input)
}

fn parse_input(input: &str) -> Vec<Instr> {
    parse_lines(parse_instr)(input).unwrap().1
}

fn simple_interpretation(instructions: &Vec<Instr>) -> (i32, i32) {
    instructions.iter().fold((0, 0), |(horizontal, depth), instr| {
        match instr {
            Forward(n) => (horizontal + n, depth),
            Down(n) => (horizontal, depth + n),
            Up(n) => (horizontal, depth - n),
        }
    })
}

fn complex_interpretation(instructions: &Vec<Instr>) -> (i32, i32, i32) {
    instructions.iter().fold((0, 0, 0), |(horizontal, depth, aim), instr| {
        match instr {
            Forward(n) => (horizontal + n, depth + aim * n, aim),
            Down(n) => (horizontal, depth, aim + n),
            Up(n) => (horizontal, depth, aim - n),
        }
    })
}

#[test]
fn example_part1() {
    let input = parse_input(EXAMPLE_INPUT);
    let (horizontal, depth) = simple_interpretation(&input);
    println!("simple: {}", horizontal * depth);
}

#[test]
fn example_part2() {
    let input = parse_input(EXAMPLE_INPUT);
    let (horizontal, depth, _) = complex_interpretation(&input);
    println!("complex: {}", horizontal * depth);
}

#[test]
fn part1() {
    let input = parse_input(INPUT);
    let (horizontal, depth) = simple_interpretation(&input);
    println!("{}", horizontal * depth);
}

#[test]
fn part2() {
    let input = parse_input(INPUT);
    let (horizontal, depth, _) = complex_interpretation(&input);
    println!("{}", horizontal * depth);
}