use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn parse(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let mut split = line.split("-");
        let cave_a = split.next().unwrap();
        let cave_b = split.next().unwrap();
        if !result.contains_key(cave_a) {
            result.insert(cave_a.to_string(), Vec::new());
        }
        result.get_mut(cave_a).unwrap().push(cave_b.to_string());
        if !result.contains_key(cave_b) {
            result.insert(cave_b.to_string(), Vec::new());
        }
        result.get_mut(cave_b).unwrap().push(cave_a.to_string());
    }

    result
}

fn walk_paths(start: &str, system: &HashMap<String, Vec<String>>, path: &mut Vec<String>) -> Vec<Vec<String>> {
    path.push(start.to_string());
    let mut paths: Vec<Vec<String>> = Vec::new();

    for connected in &system[start] {
        if connected == "start" {
            continue;
        }
        if connected != "end" {
            if connected.chars().all(|c| matches!(c, 'a'..='z')) && path.contains(connected) {
                // Skip this branch as it is lowercase and already used
                continue;
            } else {
                let mut nextpath = path.clone();
                let mut nextpaths = walk_paths(connected, system, &mut nextpath);
                for p in &mut nextpaths {
                    paths.push(p.to_vec());
                }
            }
        } else {
            path.push(connected.to_string());
            paths.push(path.to_vec());
        }
    }

    paths
}

fn star_one(lines: &Vec<String>) -> usize {
    let system = parse(lines);

    let mut path: Vec<String> = Vec::new();
    let paths = walk_paths("start", &system, &mut path);

    paths.len()
}

fn walk_paths_twice(start: &str, system: &HashMap<String, Vec<String>>, path: &mut Vec<String>, seen: &mut Vec<String>) -> Vec<Vec<String>> {
    path.push(start.to_string());
    let mut paths: Vec<Vec<String>> = Vec::new();

    for connected in &system[start] {
        if connected == "start" {
            continue;
        }
        if connected != "end" {
            if connected.chars().all(|c| matches!(c, 'a'..='z')) && path.contains(connected) && seen.len() > 0 {
                // Skip this branch as it is lowercase and already used
                continue;
            } else {
                let mut nextseen = seen.clone();
                if connected.chars().all(|c| matches!(c, 'a'..='z')) && path.contains(connected) {
                    nextseen.push(connected.to_string());
                }
                let mut nextpath = path.clone();
                let mut nextpaths = walk_paths_twice(connected, system, &mut nextpath, &mut nextseen);
                for p in &mut nextpaths {
                    paths.push(p.to_vec());
                }
            }
        } else {
            path.push(connected.to_string());
            paths.push(path.to_vec());
        }
    }

    paths
}

fn star_two(lines: &Vec<String>) -> usize {
    let system = parse(lines);

    let mut path: Vec<String> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let paths = walk_paths_twice("start", &system, &mut path, &mut seen);

    paths.len()
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
    static SIMPLE_TEST_DATA: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn simple() {
        let lines: Vec<String> = SIMPLE_TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 10);
    }


    static TEST_DATA: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 19);
    }

    #[test]
    fn simple_star_two() {
        let lines: Vec<String> = SIMPLE_TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 36);
    }


    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 103);
    }
}
