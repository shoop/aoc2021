use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
enum Direction {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn parse_line(line: &String) -> Direction {
    let mut split = line.split(" ");
    match (
        split.next().expect("Line too short"),
        split
            .next()
            .expect("Line too short")
            .parse::<isize>()
            .expect("Invalid number"),
    ) {
        ("forward", val) => Direction::Forward(val),
        ("up", val) => Direction::Up(val),
        ("down", val) => Direction::Down(val),
        _ => panic!("Invalid command"),
    }
}

fn parse_lines(lines: Vec<String>) -> Vec<Direction> {
    lines.iter().map(|line| parse_line(line)).collect()
}

fn star_one(lines: Vec<String>) -> isize {
    let directions = parse_lines(lines);
    let mut horizontal = 0;
    let mut depth = 0;

    for dir in directions.iter() {
        match dir {
            Direction::Forward(val) => horizontal += val,
            Direction::Up(val) => depth -= val,
            Direction::Down(val) => depth += val,
        }
    }

    horizontal * depth
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(lines);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(lines);
        assert_eq!(ans, 150);
    }
}
