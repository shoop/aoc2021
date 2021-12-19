use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    LeftParen,
    Number(u32),
    Comma,
    RightParen,
}

fn parse_simple(line: &str) -> Vec<Symbol> {
    let mut result: Vec<Symbol> = Vec::new();

    for ch in line.chars() {
        result.push(match ch {
            '[' => Symbol::LeftParen,
            '0'..='9' => Symbol::Number(ch.to_digit(10).unwrap()),
            ',' => Symbol::Comma,
            ']' => Symbol::RightParen,
            _ => panic!("Invalid char {} in input", ch)
        });
    }

    result
}

fn explode(num: &Vec<Symbol>) -> (Vec<Symbol>, bool) {
    let mut result: Vec<Symbol> = Vec::new();
    let mut curdepth: u32 = 0;
    let mut prev_idx: Option<usize> = None;
    let mut right_num: Option<u32> = None;
    let mut iter = num.iter().enumerate();
    let mut exploded: bool = false;
    while let Some((idx, sym)) = iter.next() {
        match sym {
            Symbol::LeftParen => {
                curdepth += 1;
                if curdepth == 5 && !exploded {
                    // Explode
                    match iter.next() {
                        Some((_, Symbol::Number(val))) => {
                            if !prev_idx.is_none() {
                                if let Symbol::Number(prev) = result[prev_idx.unwrap()] {
                                    result[prev_idx.unwrap()] = Symbol::Number(prev + val);
                                }
                            }
                        }
                        _v => panic!("invalid left side when exploding: {:?}", _v)
                    }
                    if let Some((_, Symbol::Comma)) = iter.next() {
                    } else {
                        panic!("expected comma when exploding");
                    }
                    match iter.next() {
                        Some((_, Symbol::Number(val))) => {
                            right_num = Some(*val);
                        }
                        _ => panic!("invalid right side when exploding")
                    }
                    if let Some((_, Symbol::RightParen)) = iter.next() {
                    } else {
                        panic!("expected right parenthesis when exploding");
                    }
                    result.push(Symbol::Number(0));
                    exploded = true;
                } else {
                    result.push(*sym);
                }
            },
            Symbol::RightParen => {
                result.push(*sym);
                curdepth -= 1;
            }
            Symbol::Number(val) => {
                prev_idx = Some(idx);
                if !right_num.is_none() {
                    result.push(Symbol::Number(val + right_num.unwrap()));
                    right_num = None;
                } else {
                    result.push(*sym);
                }
            },
            Symbol::Comma => result.push(*sym),
        }
    }

    (result, exploded)
}

fn split(num: &Vec<Symbol>) -> (Vec<Symbol>, bool) {
    let mut result: Vec<Symbol> = Vec::new();
    let mut splitted: bool = false;

    for sym in num.iter() {
        match sym {
            Symbol::Number(val) => {
                if *val < 10 || splitted {
                    result.push(*sym);
                } else {
                    result.push(Symbol::LeftParen);
                    result.push(Symbol::Number(*val / 2));
                    result.push(Symbol::Comma);
                    result.push(Symbol::Number(val - (*val / 2)));
                    result.push(Symbol::RightParen);
                    splitted = true;
                }
            },
            _ => result.push(*sym),
        }
    }

    (result, splitted)
}

fn add(left: &Vec<Symbol>, right: &Vec<Symbol>) -> Vec<Symbol> {
    let mut result: Vec<Symbol> = Vec::new();
    result.push(Symbol::LeftParen);
    result.extend_from_slice(left);
    result.push(Symbol::Comma);
    result.extend_from_slice(right);
    result.push(Symbol::RightParen);
    result
}

fn reduce(num: &Vec<Symbol>) -> Vec<Symbol> {
    let mut result: Vec<Symbol> = num.clone();

    loop
    {
        loop
        {
            let res = explode(&result);
            if !res.1 {
                break;
            }
            result = res.0;
        }
        let res = split(&result);
        if !res.1 {
            break;
        }
        result = res.0;
    }

    result
}

fn intern_mag(iter: &mut std::slice::Iter<Symbol>) -> u32 {
    match iter.next() {
        Some(Symbol::LeftParen) => {},
        Some(Symbol::Number(val)) => return *val,
        _ => panic!("Expected number or left parenthesis in magnitude"),
    }

    let left_mag = intern_mag(iter);
    if let Some(Symbol::Comma) = iter.next() {
    } else {
        panic!("Expected comma in magnitude");
    }
    let right_mag = intern_mag(iter);
    if let Some(Symbol::RightParen) = iter.next() {
    } else {
        panic!("Expected right parenthesis in magnitude");
    }

    left_mag * 3 + right_mag * 2
}

fn magnitude(num: &Vec<Symbol>) -> u32 {
    let mut iter = num.iter();
    intern_mag(&mut iter)
}

fn star_one(lines: &Vec<String>) -> u32 {
    let mut iter = lines.iter();
    let mut num = parse_simple(iter.next().unwrap());
    while let Some(line) = iter.next() {
        let right = parse_simple(line);
        num = add(&num, &right);
        num = reduce(&num);
    }

    magnitude(&num)
}

fn star_two(lines: &Vec<String>) -> u32 {
    let nums: Vec<Vec<Symbol>> = lines.iter().map(|l| parse_simple(l)).collect();
    let mut magns: Vec<Vec<u32>> = Vec::new();

    for y in 0..nums.len() {
        magns.push(Vec::new());
        for x in 0..nums.len() {
            if x == y {
                magns[y].push(0);
                continue;
            }

            magns[y].push(magnitude(&reduce(&add(&nums[y], &nums[x]))));
        }
    }

    *magns.iter().map(|xvec| xvec.iter().max().unwrap()).max().unwrap()
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
    static TEST_EXPLODE_1: &str = "[[[[[9,8],1],2],3],4]";

    #[test]
    fn explode_one()
    {
        let symbols = super::parse_simple(TEST_EXPLODE_1);
        let result = super::explode(&symbols);
        assert_eq!(result.1, true);
        let symbols = result.0;
        assert_eq!(symbols[4], super::Symbol::Number(0));
        assert_eq!(symbols[6], super::Symbol::Number(9));
    }

    static TEST_EXPLODE_2: &str = "[[6,[5,[4,[3,2]]]],1]";

    #[test]
    fn explode_two()
    {
        let symbols = super::parse_simple(TEST_EXPLODE_2);
        let result = super::explode(&symbols);
        assert_eq!(result.1, true);
        let symbols = result.0;
        assert_eq!(symbols[8], super::Symbol::Number(7));
        assert_eq!(symbols[10], super::Symbol::Number(0));
        assert_eq!(symbols[15], super::Symbol::Number(3));
    }

    static TEST_REDUCE: &str = "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]";

    #[test]
    fn reduce()
    {
        let left = super::parse_simple(TEST_REDUCE.lines().nth(0).unwrap());
        let right = super::parse_simple(TEST_REDUCE.lines().nth(1).unwrap());
        let num = super::add(&left, &right);
        let red = super::reduce(&num);
        assert_eq!(red[4], super::Symbol::Number(0));
        assert_eq!(red[14], super::Symbol::Number(7));
    }

    static TEST_MAGNITUDE: &str = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

    #[test]
    fn magnitude()
    {
        let num = super::parse_simple(TEST_MAGNITUDE);
        let mag = super::magnitude(&num);
        assert_eq!(mag, 3488);
    }

    static TEST_DATA: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 4140);
    }
}
