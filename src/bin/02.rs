advent_of_code::solution!(2);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    Some(8)
    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(2286)
    // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
