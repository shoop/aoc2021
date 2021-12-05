use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct Number {
    nr: usize,
    marked: bool,
}

#[derive(Debug)]
struct Board {
    nrs: Vec<Vec<Number>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in &self.nrs {
            for x in y {
                write!(f, "{num:>2} ", num = x.nr)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Board {
    fn mark(&mut self, num: usize) {
        for y in &mut self.nrs {
            for x in &mut (*y) {
                if x.nr == num {
                    x.marked = true;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        // Lines
        for y in &self.nrs {
            if y.iter().fold(true, |acc, num| acc & num.marked) == true {
                return true;
            }
        }

        // Columns
        for x in 0..self.nrs[0].len() {
            if (0..self.nrs.len()).fold(true, |acc, y| acc & self.nrs[y][x].marked) == true {
                return true;
            }
        }

        false
    }

    fn calc_score(&self, last_ans: usize) -> usize {
        last_ans
            * self.nrs.iter().fold(0, |yacc, xvec| {
                yacc + xvec
                    .iter()
                    .fold(0, |xacc, num| if num.marked { xacc } else { xacc + num.nr })
            })
    }
}

// TODO: figure out how to extract the parse_board while consuming the iterator

// fn parse_board<'a, I>(lines: I) -> Board
// where
//     I: Iterator<Item = &'a &'a str> + Clone,
// {
//     let mut board = Board { nrs: Vec::new() };
//     for (y, line) in lines.enumerate() {
//         if y == 5 {
//             break;
//         }
//         board.nrs.push(Vec::new());
//         for num in line.split_whitespace() {
//             board.nrs[y].push(Number { nr: num.parse::<usize>().expect("Invalid line"), marked: false });
//         }
//     }

//     board
// }

fn star_one(lines: &Vec<&str>) -> usize {
    let mut boards: Vec<Board> = Vec::new();

    let mut iter = lines.iter();
    let line = iter.next().expect("At least one line");
    let answers: Vec<usize> = line
        .split(',')
        .map(|x| x.parse::<usize>().expect("Invalid answer"))
        .collect();
    iter.next();
    loop {
        let mut board = Board { nrs: Vec::new() };
        for y in 0..5 {
            if let Some(line) = iter.next() {
                if line.is_empty() {
                    break;
                }
                board.nrs.push(Vec::new());
                for num in line.split_whitespace() {
                    board.nrs[y].push(Number {
                        nr: num.parse::<usize>().expect("Invalid line"),
                        marked: false,
                    });
                }
            }
        }
        boards.push(board);
        if iter.next() == None {
            break;
        }
    }

    for ans in answers {
        for board in boards.iter_mut() {
            board.mark(ans);
            if board.has_won() {
                return board.calc_score(ans);
            }
        }
    }

    0
}

fn star_two(lines: &Vec<&str>) -> usize {
    let mut boards: Vec<Board> = Vec::new();

    let mut iter = lines.iter();
    let line = iter.next().expect("At least one line");
    let answers: Vec<usize> = line
        .split(',')
        .map(|x| x.parse::<usize>().expect("Invalid answer"))
        .collect();
    iter.next();
    loop {
        let mut board = Board { nrs: Vec::new() };
        for y in 0..5 {
            if let Some(line) = iter.next() {
                if line.is_empty() {
                    break;
                }
                board.nrs.push(Vec::new());
                for num in line.split_whitespace() {
                    board.nrs[y].push(Number {
                        nr: num.parse::<usize>().expect("Invalid line"),
                        marked: false,
                    });
                }
            }
        }
        boards.push(board);
        if iter.next() == None {
            break;
        }
    }

    let board_total = boards.len();
    let mut board_count: usize = 0;
    for ans in answers {
        for board in boards.iter_mut() {
            if !board.has_won() {
                board.mark(ans);
                if board.has_won() {
                    board_count += 1;
                    if board_count == board_total {
                        return board.calc_score(ans);
                    }
                }
            }
        }
    }

    0
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines.iter().map(|x| x.as_str()).collect());
    println!("Star one: {}", ans);

    let ans = star_two(&lines.iter().map(|x| x.as_str()).collect());
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines.iter().map(|x| x.as_str()).collect());
        assert_eq!(ans, 4512);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines.iter().map(|x| x.as_str()).collect());
        assert_eq!(ans, 1924);
    }
}
