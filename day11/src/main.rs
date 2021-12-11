use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<usize>>) {
    for y in map {
        for x in y {
            if *x < 10 {
                print!("{}", x);
            }
            if *x == 10 {
                print!("X");
            }
            if *x > 10 {
                print!("o");
            }
        }
        println!();
    }
}

fn step(map: &mut Vec<Vec<usize>>) -> usize {
    // Increase energy level
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map[y][x] = map[y][x] + 1;
        }
    }

    // Flash anything > 9 once
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    loop
    {
        let mut flash: Option<(usize, usize)> = None;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] > 9 && !flashed.contains(&(y, x)) {
                    flash = Some((y, x));
                    break;
                }
            }
        }

        if let Some((y, x)) = flash {
            flashed.insert((y, x));
            if y > 0 && x > 0 {
                map[y-1][x-1] = map[y-1][x-1] + 1;
            }
            if y > 0 {
                map[y-1][x] = map[y-1][x] + 1;
            }
            if y > 0 && x < map[0].len() -1 {
                map[y-1][x+1] = map[y-1][x+1] + 1;
            }
            if x > 0 {
                map[y][x-1] = map[y][x-1] + 1;
            }
            if x < map[0].len() - 1 {
                map[y][x+1] = map[y][x+1] + 1;
            }
            if y < map.len() - 1 && x > 0 {
                map[y+1][x-1] = map[y+1][x-1] + 1;
            }
            if y < map.len() - 1 {
                map[y+1][x] = map[y+1][x] + 1;
            }
            if y < map.len() - 1 && x < map[0].len() - 1 {
                map[y+1][x+1] = map[y+1][x+1] + 1;
            }
        } else {
            break;
        }
    }

    // Set flashed to 0
    for coord in &flashed {
        map[coord.0][coord.1] = 0;
    }

    // Return count of flashes
    flashed.iter().count()
}

fn parse(lines: &Vec<String>) -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
    }

    map
}

fn star_one(lines: &Vec<String>) -> usize {
    let mut map = parse(lines);

    let mut flashcount = 0;
    for _ in 0..100 {
        flashcount += step(&mut map);
    }

    flashcount
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
    static SIMPLE_TEST_DATA: &str = "11111
19991
19191
19991
11111";

    #[test]
    fn test_simple_example() {
        let lines: Vec<String> = SIMPLE_TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let mut map = super::parse(&lines);
        println!("=== 0");
        super::print_map(&map);
        let ans = super::step(&mut map);
        println!("=== 1");
        super::print_map(&map);
        super::step(&mut map);
        println!("=== 2");
        super::print_map(&map);
        assert_eq!(ans, 9);
    }

    static TEST_DATA: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 1656);
    }
}
