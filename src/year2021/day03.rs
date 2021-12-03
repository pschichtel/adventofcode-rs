use nom::{IResult, Parser};
use crate::helpers::{parse_lines, parse_binary_number};

const INPUT: &str = include_str!("inputs/day03.txt");
const EXAMPLE_INPUT: &str = include_str!("inputs/day03_example.txt");

fn parse_bits(input: &str) -> IResult<&str, u32> {
    parse_binary_number(input).map(|(rest, i)| (rest, i as u32))
}

fn parse_input(input: &str) -> Vec<u32> {
    parse_lines(parse_bits).parse(input).unwrap().1
}

fn count_ones_per_bit(input: &Vec<u32>, bits: usize) -> (Vec<usize>, Vec<u32>) {
    let mut ones = vec![0; bits];
    for number in input {
        for i in 0..bits {
            let bit = number >> i & 1;
            let index = bits - 1 - i;
            if bit == 1 {
                ones[index] += 1;
            }
        }
    }

    let input_size = input.len();
    let common_per_bit: Vec<u32> = ones
        .iter()
        .map(|ones| if *ones as f32 >= (input_size as f32 / 2.0) { 1 } else { 0 })
        .collect();

    (ones, common_per_bit)
}

fn calculate_gamma_rate(input: &Vec<u32>, ones: &Vec<usize>, bits: usize) -> u32 {
    let half_input_len = input.len() / 2;
    let mut gamma_rate = 0;
    for i in 0..bits {
        if ones[i] > half_input_len {
            gamma_rate |= 1 << (bits - 1 - i)
        }
    }
    gamma_rate
}

fn calculate_epsilon_rate(gamma_rate: u32, bits: usize) -> u32 {
    !gamma_rate & ((u32::MAX << bits) ^ u32::MAX)
}

fn sieve_numbers_by_bit(input: &Vec<u32>, bits: usize, filter: impl Fn(u32, u32) -> bool) -> Option<u32> {
    fn sieve(input: &Vec<u32>, i: usize, bits: usize, filter: impl Fn(u32, u32) -> bool) -> Option<u32> {
        let (_, common_per_bit) = count_ones_per_bit(input, bits);
        let common_value = common_per_bit[i];
        let keep = input
            .iter()
            .copied()
            .filter(|n| { filter((n >> (bits - 1 - i)) & 1, common_value) })
            .collect::<Vec<u32>>();
        match keep.len() {
            0 => None,
            1 => Some(keep[0]),
            _ => sieve(&keep, i + 1, bits, filter),
        }
    }

    sieve(input, 0, bits, filter)
}

fn calculate_oxygen_generator_rating(input: &Vec<u32>, bits: usize) -> Option<u32> {
    sieve_numbers_by_bit(&input, bits, |bit, common_value| {
        bit == common_value
    })
}

fn calculate_co2_scrubber_rating(input: &Vec<u32>, bits: usize) -> Option<u32> {
    sieve_numbers_by_bit(&input, bits, |bit, common_value| {
        bit != common_value
    })
}

fn calculate_power_consumption(input: &Vec<u32>, bits: usize) -> u32 {
    let (ones_per_bit, _) = count_ones_per_bit(input, bits);
    let gamma_rate = calculate_gamma_rate(input, &ones_per_bit, bits);
    let epsilon_rate = calculate_epsilon_rate(gamma_rate, bits);
    gamma_rate * epsilon_rate
}

fn calculate_life_support_rating(input: &Vec<u32>, bits: usize) -> u32 {
    let oxygen_generator_rating = calculate_oxygen_generator_rating(&input, bits);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(&input, bits);
    oxygen_generator_rating.expect("oxygen rating not found!") * co2_scrubber_rating.expect("co2 scrubber rating not found!")
}

#[test]
fn part1_example() {
    let input = parse_input(EXAMPLE_INPUT);
    println!("{:?}", calculate_power_consumption(&input, 5));
    println!("{:?}", calculate_life_support_rating(&input, 5));
}

#[test]
fn part1() {
    let input = parse_input(INPUT);
    println!("{:?}", calculate_power_consumption(&input, 12));
}

#[test]
fn part2() {
    let input = parse_input(INPUT);
    println!("{:?}", calculate_life_support_rating(&input, 12));
}