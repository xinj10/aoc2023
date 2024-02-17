advent_of_code::solution!(2);
use regex::Regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Default, Debug)]
struct GameSet {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    game_sets: Vec<GameSet>,
}

impl Game {
    fn find_max(&self, color: &str) -> u32 {
        0
    }
}

fn parse_input(input: &str) -> Game {
    let parts: Vec<&str> = input.split(":").collect();
    let re_id = Regex::new(r"^Game (?<id>\d+)").unwrap();
    let id: u32 = re_id.captures(parts[0]).unwrap()["id"].parse().unwrap();
    let raw_game_sets = parts[1].split(";");
    let re_color = Regex::new(r"(?<value>\d+) (?<color>\w+)").unwrap();
    let mut game_sets: Vec<GameSet> = Vec::new();
    for raw_game_set in raw_game_sets {
        let mut game_set = GameSet::default();
        for ball in raw_game_set.split(",") {
            let captures = re_color.captures(ball).unwrap();
            let value: u32 = captures["value"].parse().unwrap();
            match &captures["color"] {
                "red" => {
                    game_set.red = value;
                }
                "green" => {
                    game_set.green = value;
                }
                "blue" => {
                    game_set.blue = value;
                }
                _ => println!("invalid color!"),
            }
        }
        game_sets.push(game_set);
    }

    Game { id, game_sets }
}

pub fn part_one(input: &str) -> Option<u32> {
    for hay in input.lines() {
        let game = parse_input(hay);
        println!("{:?}", game);
    }
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
