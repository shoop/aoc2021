use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
enum PairType {
    Pair,
    Number(u32),
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    pair_type: PairType,
    depth: u32,
    // TODO: convert to Option<Vec<Pair>> as for numbers we don't need this
    // TODO: alternatively, figure out how to do recursive enum and put the left,right pair into the PairType
    subpairs: Vec<Pair>,
}

fn parse_pair(iter: &mut std::str::Chars, depth: u32) -> Pair {
    let mut result: Pair = Pair { pair_type: PairType::Pair, depth: depth, subpairs: Vec::new() };
    let ch = iter.next().unwrap();
    match ch {
        '[' => {
            result.subpairs.push(parse_pair(iter, depth + 1));
        },
        '0'..='9' => {
            result.subpairs.push(Pair { pair_type: PairType::Number(ch.to_digit(10).unwrap()), depth: depth, subpairs: Vec::new() });
        },
        _ => panic!("Invalid char for left side of pair {}", ch)
    }
    let ch = iter.next().unwrap();
    if ch != ',' {
        panic!("Expected comma separator for pair");
    }
    let ch = iter.next().unwrap();
    match ch {
        '[' => {
            result.subpairs.push(parse_pair(iter, depth + 1));
        },
        '0'..='9' => {
            result.subpairs.push(Pair { pair_type: PairType::Number(ch.to_digit(10).unwrap()), depth: depth, subpairs: Vec::new() });
        },
        _ => panic!("Invalid char for left side of pair {}", ch)
    }
    let ch = iter.next().unwrap();
    if ch != ']' {
        panic!("Expected right parenthesis to end atom");
    }
    
    result
}

fn parse_line(line: &str) -> Pair {
    let mut iter = line.chars();
    let ch = iter.next().unwrap();
    if ch != '[' {
        panic!("Expected start of pair at start of line");
    }

    parse_pair(&mut iter, 0)
}

fn star_one(_lines: &Vec<String>) -> usize {
    let _pair = parse_line(&_lines[0]);

    0
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static PARSE_TEST_DATA: &str = "[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

    #[test]
    fn parse() {
        let pair = super::parse_line(PARSE_TEST_DATA.lines().nth(0).unwrap());
        assert_eq!(pair.pair_type, super::PairType::Pair);
        assert_eq!(pair.depth, 0);
        assert_eq!(pair.subpairs[0].pair_type, super::PairType::Number(1));
        assert_eq!(pair.subpairs[0].depth, 0);
        assert_eq!(pair.subpairs[1].pair_type, super::PairType::Number(2));
        assert_eq!(pair.subpairs[1].depth, 0);

        let pair = super::parse_line(PARSE_TEST_DATA.lines().nth(1).unwrap());
        assert_eq!(pair.pair_type, super::PairType::Pair);
        assert_eq!(pair.subpairs[0].pair_type, super::PairType::Pair);
        assert_eq!(pair.subpairs[0].depth, 1);
        assert_eq!(pair.subpairs[0].subpairs[0].pair_type, super::PairType::Number(1));
        assert_eq!(pair.subpairs[0].subpairs[0].depth, 1);
        assert_eq!(pair.subpairs[0].subpairs[1].pair_type, super::PairType::Number(2));
        assert_eq!(pair.subpairs[0].subpairs[1].depth, 1);
        assert_eq!(pair.subpairs[1].pair_type, super::PairType::Number(3));
        assert_eq!(pair.subpairs[1].depth, 0);
    }

    static TEST_DATA: &str = "";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 100);
    }
}
