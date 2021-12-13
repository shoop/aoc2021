use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug, PartialEq)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: FoldAxis,
    pos: usize,
}

#[derive(Debug)]
struct Paper {
    dots: Vec<Vec<bool>>,
}

#[allow(dead_code)]
fn print_paper(paper: &Paper) {
    for xvec in &paper.dots {
        for dot in xvec {
            print!("{}", if *dot == true { "#" } else { "."})
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_folds(folds: &Vec<Fold>) {
    for f in folds {
        println!("fold over {}={}", if f.axis == FoldAxis::X { "x" } else { "y" }, f.pos);
    }
}

fn parse(lines: &Vec<String>) -> (Paper, Vec<Fold>) {
    let mut paper: Paper = Paper { dots: Vec::new() };
    let mut folds: Vec<Fold> = Vec::new();
    let mut iter = lines.iter();

    // Paper
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }

        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();

        if y >= paper.dots.len() {
            for _ in paper.dots.len()..y+1 {
                paper.dots.push(Vec::new());
            }
        }

        if x >= paper.dots[y].len() {
            for _ in paper.dots[y].len()..x+1 {
                paper.dots[y].push(false);
            }
        }

        paper.dots[y][x] = true;
    }

    // Ensure rectangular map
    let max_x = paper.dots.iter().map(|xvec| xvec.len()).max().unwrap();
    for xvec in &mut paper.dots {
        for _ in xvec.len()..max_x {
            xvec.push(false);
        }
    }

    // Folds
    while let Some(line) = iter.next() {
        let mut split = line[11..].split('=');
        let axis = match split.next().unwrap().chars().next().unwrap() {
            'y' => FoldAxis::Y,
            'x' => FoldAxis::X,
            _ => panic!("Invalid fold axis"),
        };
        let pos = split.next().unwrap().parse::<usize>().unwrap();
        folds.push(Fold { axis: axis, pos: pos });
    }

    (paper, folds)
}

fn perform_fold(paper: &mut Paper, fold: &Fold) {
    let max_y = paper.dots.len();
    let max_x = paper.dots[0].len();

    match fold.axis {
        FoldAxis::Y => {
            // Perform fold
            let mut offs: usize = 1;
            while (fold.pos + offs < max_y) && (fold.pos >= offs) {
                for x in 0..max_x {
                    if paper.dots[fold.pos + offs][x] {
                        paper.dots[fold.pos - offs][x] = true;
                    }
                }
                offs += 1;
            }

            // Remove folded lines
            paper.dots.truncate(fold.pos);
        },
        FoldAxis::X => {
            // Perform fold
            let mut offs: usize = 1;
            while (fold.pos + offs < max_x) && (fold.pos >= offs) {
                for y in 0..max_y {
                    if paper.dots[y][fold.pos + offs] {
                        paper.dots[y][fold.pos - offs] = true;
                    }
                }
                offs += 1;
            }

            // Remove folded lines
            for xvec in &mut paper.dots {
                xvec.truncate(fold.pos);
            }
        },
    }
}

fn count_dots(paper: &Paper) -> usize {
    paper.dots.iter().fold(0, |acc, xvec| acc + xvec.iter().fold(0, |acc, val| if *val { acc + 1 } else { acc }))
}

fn star_one(lines: &Vec<String>) -> usize {
    let parse_result = parse(lines);
    let mut paper = parse_result.0;
    let folds = parse_result.1;

    perform_fold(&mut paper, &folds[0]);

    count_dots(&paper)
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
    static TEST_DATA: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 17);
    }
}
