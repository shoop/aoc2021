use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn flood_fill(map: &Vec<Vec<u32>>, basinmap: &mut Vec<Vec<u32>>, y: usize, x: usize, basin: u32) {
    basinmap[y][x] = basin;

    // up
    if y > 0 && map[y-1][x] != 9 && basinmap[y-1][x] == 0 {
        flood_fill(map, basinmap, y-1, x, basin);
    }

    // left
    if x > 0 && map[y][x-1] != 9 && basinmap[y][x-1] == 0 {
        flood_fill(map, basinmap, y, x-1, basin);
    }

    // right
    if x < map[y].len() - 1 && map[y][x+1] != 9 && basinmap[y][x+1] == 0 {
        flood_fill(map, basinmap, y, x+1, basin);
    }
    // down
    if y < map.len() - 1 && map[y+1][x] != 9 && basinmap[y+1][x] == 0 {
        flood_fill(map, basinmap, y+1, x, basin);
    }
}

fn star_two(lines: &Vec<String>) -> u32 {
    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut basinmap: Vec<Vec<u32>> = vec![vec![0; map[0].len()]; map.len()];
    let mut nextbasin: u32 = 1;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 9 {
                continue;
            }

            if basinmap[y][x] != 0 {
                continue;
            }

            flood_fill(&map, &mut basinmap, y, x, nextbasin);
            nextbasin += 1;
        }
    }

    let mut basincounts: Vec<u32> = Vec::new();
    for basin in 1..nextbasin {
        basincounts.push(basinmap.iter().fold(0, |acc, v| acc + v.iter().filter(|x| **x == basin).count() as u32));
    }
    basincounts.sort_by(|a, b| a.cmp(b).reverse());
    basincounts.truncate(3);

    basincounts.iter().fold(1, |acc, x| acc * x)
}

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

    let ans = star_two(&lines);
    println!("Star two: {}", ans);
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

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 1134);
    }
}
