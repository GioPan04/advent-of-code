use std::{char, collections::HashMap};

advent_of_code::solution!(1);

const DIGITS: [&str; 20] = [
    "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
    "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn get_digit_one(chars: impl Iterator<Item = char>) -> Option<char> {
    for char in chars {
        if char.is_numeric() {
            return Some(char);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let first = get_digit_one(line.chars()).unwrap();
        let last = get_digit_one(line.chars().rev()).unwrap();

        let mut string = String::new();
        string.push(first);
        string.push(last);

        total += string.parse::<u32>().unwrap();
    }

    Some(total)
}

fn parse_line(input: &str) -> Vec<u32> {
    let mut digits = HashMap::<usize,u32>::new();
    let input = input.to_string();

    for (digit_index, digit) in DIGITS.iter().enumerate() {
        let matches = input.match_indices(digit);

        for (index, _) in matches {
            digits.insert(index, (digit_index % 10) as u32);
        }

    }

    let mut sorted: Vec<_> = digits.into_iter().collect();
    sorted.sort_by_key(|&(key, _)| key);

    sorted.into_iter().map(|(_, value)| value).collect()

}

pub fn part_two(input: &str) -> Option<u32> {

    let mut total = 0;
    for line in input.lines() {
        let line = line;
        let first = parse_line(&line.clone()).first().unwrap().clone();
        let last = parse_line(line.clone()).last().unwrap().clone();

        total += (first * 10) + last;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
