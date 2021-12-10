use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
enum ParseResult {
    Illegal(char),
    Incomplete(Vec<char>),
    Ok,
}

fn parse_line(line: &str) -> ParseResult {
    let mut parsestack: Vec<char> = Vec::new();

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => {
                parsestack.push(ch);
                continue;
            }
            ')' => {
                match parsestack.pop() {
                    Some('(') => continue,
                    Some('[') | Some('{') | Some('<') => return ParseResult::Illegal(ch),
                    None => panic!("No character on stack"),
                    _ => panic!("Invalid character on stack"),
                }
            }
            ']' => {
                match parsestack.pop() {
                    Some('[') => continue,
                    Some('(') | Some('{') | Some('<') => return ParseResult::Illegal(ch),
                    None => panic!("No character on stack"),
                    _ => panic!("Invalid character on stack"),
                }
            }
            '}' => {
                match parsestack.pop() {
                    Some('{') => continue,
                    Some('[') | Some('(') | Some('<') => return ParseResult::Illegal(ch),
                    None => panic!("No character on stack"),
                    _ => panic!("Invalid character on stack"),
                }
            }
            '>' => {
                match parsestack.pop() {
                    Some('<') => continue,
                    Some('[') | Some('{') | Some('(') => return ParseResult::Illegal(ch),
                    None => panic!("No character on stack"),
                    _ => panic!("Invalid character on stack"),
                }
            }
            _ => panic!("Invalid character in input"),
        }
    }

    if parsestack.len() > 0 {
        return ParseResult::Incomplete(parsestack.clone());
    }

    ParseResult::Ok
}

fn star_one(lines: &Vec<String>) -> usize {
    let mut total: usize = 0;
    for line in lines {
        match parse_line(line) {
            ParseResult::Illegal(ch) => match ch {
                ')' => total += 3,
                ']' => total += 57,
                '}' => total += 1197,
                '>' => total += 25137,
                _ => panic!("Character {} not scored", ch)
            },
            ParseResult::Incomplete(_) => {},
            ParseResult::Ok => {},
        }
    }

    total
}

fn star_two(lines: &Vec<String>) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    for line in lines {
        match parse_line(line) {
            ParseResult::Illegal(_) => {},
            ParseResult::Incomplete(chars) => {
                scores.push(chars.iter().rev().fold(0, |acc, ch| {
                    acc * 5 + match ch {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Invalid character {} on stack", ch),
                    }
                }));
            },
            ParseResult::Ok => panic!("OK line found, not expected"),
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines);
    println!("Star one: {}", ans);

    let ans = star_two(&lines);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 26397);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 288957);
    }    
}
