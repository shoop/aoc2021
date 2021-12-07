use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(lines: &Vec<String>) -> i32 {
    let crabs: Vec<i32> = lines[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let &max = crabs.iter().max().unwrap();
    let &min = crabs.iter().min().unwrap();

    let mut possibles: Vec<i32> = vec![0; (max as usize) + 1];
    for i in min..(max + 1) {
        possibles[(i as usize)] = crabs.iter().map(|x| i32::abs(i - x)).sum()
    }

    *possibles.iter().filter(|x| **x != 0).min().unwrap()
}

fn star_two(lines: &Vec<String>) -> i32 {
    let crabs: Vec<i32> = lines[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let &max = crabs.iter().max().unwrap();
    let &min = crabs.iter().min().unwrap();

    let mut possibles: Vec<i32> = vec![0; (max as usize) + 1];
    for i in min..(max + 1) {
        possibles[(i as usize)] = crabs
            .iter()
            .map(|x| {
                let upper = i32::abs(i - x);
                let lower = 1;
                let total = (((lower as f64 + upper as f64) / 2 as f64) * upper as f64) as i32;
                total
            })
            .sum();
    }

    *possibles.iter().filter(|x| **x != 0).min().unwrap()
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
    static TEST_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 37);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 168);
    }
}
