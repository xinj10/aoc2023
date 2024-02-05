advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;
    for line in lines {
        let mut numbers: Vec<u32> = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                numbers.push(char.to_digit(10).unwrap());
            }
        }
        match numbers.len() {
            0 => continue,
            _ => sum += numbers[0] * 10 + numbers[numbers.len() - 1],
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_table: HashMap<&str, u32> = HashMap::from([
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
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0u32;
    for line in lines {
        let numbers = parse_line(line, &number_table);
        match numbers.len() {
            0 => continue,
            _ => sum += numbers[0] * 10 + numbers[numbers.len() - 1],
        }
    }
    None
}

fn parse_line(line: &str, number_table: &HashMap<&str, u32>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let total_len = line.len();
    for (i, symbol) in line.chars().enumerate() {
        if symbol.is_digit(10) {
            numbers.push(symbol.to_digit(10).unwrap());
        } else {
            println!("here!");
            for number_str in number_table.keys() {
                let number_str_len = number_str.len();
                if total_len - i < number_str_len {
                    // not long enough
                    continue;
                }
                if line.to_string()[i..(i + number_str_len - 1)] == **number_str {
                    numbers.push(*number_table.get(number_str).unwrap())
                }
            }
        }
    }
    return numbers;
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
        assert_eq!(result, None);
    }
}
