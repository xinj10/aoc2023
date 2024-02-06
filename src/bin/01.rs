advent_of_code::solution!(1);
use regex::Regex;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    //TODO: use regex
    let lines: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;
    let re = Regex::new(r"\d").unwrap();
    for hay in lines {
        let numbers: Vec<u32> = re
            .find_iter(hay)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        match numbers.len() {
            0 => continue,
            _ => sum += numbers[0] * 10 + numbers[numbers.len() - 1],
        }
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let _number_table: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
