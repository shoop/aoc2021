use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn astar(map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    let maxy = map.len();
    let maxx = map[0].len();

    let mut open_set: HashSet<(usize, usize)> = HashSet::new();
    open_set.insert((0, 0));
    
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut gscore: HashMap<(usize, usize), usize> = HashMap::new();
    let mut fscore: HashMap<(usize, usize), usize> = HashMap::new();
    for y in 0..maxy {
        for x in 0..maxx {
            gscore.insert((y, x), usize::MAX);
            fscore.insert((y, x), usize::MAX);
        }
    }
    *gscore.get_mut(&(0, 0)).unwrap() = 0;
    *fscore.get_mut(&(0, 0)).unwrap() = 0;

    while open_set.len() > 0 {
        let mut cur = open_set.iter().min_by(|a, b| fscore.get(a).cmp(&fscore.get(b))).unwrap().clone();
        if cur.0 == maxy - 1 && cur.1 == maxx - 1 {
            // Arrived at destination, reconstruct path
            result.push(cur);
            while came_from.contains_key(&cur) {
                cur = came_from[&cur];
                result.insert(0, cur)
            }
            return result;
        }

        open_set.remove(&cur);
        for (y, x) in vec![(0usize, 1usize), (1, 0)] {
            if cur.0 + y == maxy || cur.1 + x == maxx {
                continue;
            }
            let neighbor = (cur.0+y, cur.1+x);
            let tentative_gscore = gscore[&cur] + map[cur.0+y][cur.1+x];
            if tentative_gscore < gscore[&neighbor] {
                *came_from.entry(neighbor).or_insert((0, 0)) = cur;
                *gscore.get_mut(&neighbor).unwrap() = tentative_gscore;
                *fscore.get_mut(&neighbor).unwrap() = tentative_gscore + (maxy - cur.0) + (maxx - cur.1);
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        }
    }

    panic!("no path found")
}

fn parse(lines: &Vec<String>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        result.push(line.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect());
    }

    result
}

fn star_one(lines: &Vec<String>) -> usize {
    let map = parse(lines);
    let path = astar(&map);
    
    path.iter().map(|(y, x)| map[*y][*x]).sum::<usize>() - map[0][0]
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
    static TEST_DATA: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 40);
    }
}
