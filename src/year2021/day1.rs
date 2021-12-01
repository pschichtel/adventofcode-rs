use crate::helpers::{parse_lines, parse_number};

const INPUT: &str = include_str!("inputs/day01.txt");

fn parse_input(input: &str) -> Vec<i32> {
    parse_lines(parse_number)(input).unwrap().1
}

fn count_raises(input: &Vec<i32>) -> usize {
    input.windows(2).map(|a| a[1] - a[0]).filter(|a| *a > 0).count()
}

#[test]
fn part1() {
    let input = parse_input(INPUT);
    println!("{}", count_raises(&input).to_string());
}

#[test]
fn part2() {
    let input = parse_input(INPUT);
    let summed_inputs: Vec<i32> = input.windows(3).map(|a| a.iter().sum()).collect();
    println!("{}", count_raises(&summed_inputs).to_string());
}