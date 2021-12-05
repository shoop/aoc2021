use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::fmt;
use std::cmp;

#[derive(Debug)]
struct Field {
    pos: Vec<Vec<usize>>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in &self.pos {
            for x in y {
                if *x > 0 {
                    write!(f, "{num:>1}", num = x)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Field {
    fn walk(&mut self, startx: usize, starty: usize, endx: usize, endy: usize) {
        // Extend field if necessary
        let maxy = cmp::max(starty, endy);
        let maxx = cmp::max(startx, endx);
        for yvec in &mut self.pos {
            while yvec.len() <= maxx {
                yvec.push(0);
            }
        }
        while self.pos.len() <= maxy {
            let newline = vec![0; maxx + 1];
            self.pos.push(newline);
        }

        // Do the walk
        let mut y = starty;
        let mut x = startx;
        while y != endy || x != endx {
            self.pos[y][x] += 1;
            if y != endy {
                if y > endy {
                    y -= 1;
                } else {
                    y += 1;
                }
            }
            if x != endx {
                if x > endx {
                    x -=1;
                } else {
                    x += 1;
                }
            }
        }
        self.pos[y][x] += 1;
    }

    fn count_bigger(&self, target: usize) -> usize {
        self.pos.iter().fold(0, |acc, xvec| acc + xvec.iter().fold(0, |acc, nr| if *nr >= target { acc + 1 } else { acc }))
    }
}

fn parse_field(lines: &Vec<String>, count_diagonal: bool) -> Field {
    let mut field = Field { pos: Vec::new() };

    for line in lines {
        let mut linespec = line.split(" -> ");
        let mut start = linespec.next().expect("Missing start coord").split(",");
        let startx = start.next().expect("Missing start X").parse::<usize>().expect("Start X not a number");
        let starty = start.next().expect("Missing start Y").parse::<usize>().expect("Start Y not a number");
        let mut end = linespec.next().expect("Missing end coord").split(",");
        let endx = end.next().expect("Missing end X").parse::<usize>().expect("eND X not a number");
        let endy = end.next().expect("Missing end Y").parse::<usize>().expect("End Y not a number");
        if count_diagonal || startx == endx || starty == endy {
            field.walk(startx, starty, endx, endy)
        }
    }

    field
}

fn star_one(lines: &Vec<String>) -> usize {
    let field = parse_field(lines, false);
    field.count_bigger(2)
}

fn star_two(lines: &Vec<String>) -> usize {
    let field = parse_field(lines, true);
    field.count_bigger(2)
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
    static TEST_DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 5);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 12);
    }
}
