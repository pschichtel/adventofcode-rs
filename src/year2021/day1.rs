
#[cfg(test)]
mod tests {
    use crate::helpers::{parse_lines, parse_number, print_solutions};

    const INPUT: &str = include_str!("inputs/day01.txt");

    fn parse_input(input: &str) -> Vec<i32> {
        parse_lines(parse_number)(input).unwrap().1
    }

    fn count_raises(input: &Vec<i32>) -> usize {
        input.windows(2).map(|a| a[1] - a[0]).filter(|a| *a > 0).count()
    }

    pub fn day1(raw_input: &str) -> (String, String) {

        let input = parse_input(raw_input);
        let summed_inputs: Vec<i32> = input.windows(3).map(|a| a.iter().sum()).collect();

        (count_raises(&input).to_string(), count_raises(&summed_inputs).to_string())
    }

    #[test]
    fn run() {
        let (part1, part2) = day1(INPUT);
        print_solutions("Day 1", part1.as_str(), part2.as_str())
    }
}