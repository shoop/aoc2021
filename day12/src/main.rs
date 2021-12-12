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
    print!("walk from {}: ", start);

    for connected in &system[start] {
        if connected == "start" {
            continue;
        }
        if connected != "end" {
            if connected.chars().all(|c| matches!(c, 'a'..='z')) && path.contains(connected) {
                print!("skipping {} as already been used this path, ", connected);
                // Skip this branch as it is lowercase and already used
                continue;
            } else {
                println!("to {}", connected);
                let mut nextpath = path.clone();
                let mut nextpaths = walk_paths(connected, system, &mut nextpath);
                for p in &mut nextpaths {
                    println!("- found {}", p.join(", "));
                    paths.push(p.to_vec());
                }
                print!("walk from {}: ", start);
            }
        } else {
            println!("to end");
            path.push(connected.to_string());
            paths.push(path.to_vec());
        }
    }

    paths
}

fn star_one(lines: &Vec<String>) -> usize {
    let system = parse(lines);
    println!("system {:?}", system);

    let mut path: Vec<String> = Vec::new();
    let paths = walk_paths("start", &system, &mut path);

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
}
