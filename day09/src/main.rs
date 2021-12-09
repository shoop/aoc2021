use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(lines: &Vec<String>) -> u32 {
    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut lowpoints: Vec<(usize,usize)> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut lowest = true;
            // up
            if y > 0 {
                lowest = lowest && (map[y-1][x] > map[y][x]);
            }
            // left
            if x > 0 {
                lowest = lowest && (map[y][x-1] > map[y][x]);
            }
            // right
            if x < map[y].len() - 1 {
                lowest = lowest && (map[y][x+1] > map[y][x]);
            }
            // down
            if y < map.len() - 1 {
                lowest = lowest && (map[y+1][x] > map[y][x]);
            }

            if lowest {
                lowpoints.push((y as usize, x as usize));
            }
        }
    }

    lowpoints.iter().map(|(y, x)| map[*y][*x] + 1).sum()
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
    static TEST_DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 15);
    }
}
