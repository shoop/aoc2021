use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
pub struct LanternFish {
    age: usize,
}

impl LanternFish {
    fn new(age: usize) -> LanternFish {
        LanternFish { age: age }
    }

    fn day_passes(&mut self) -> bool {
        if self.age == 0 {
            self.age = 6;
            return true;
        } else {
            self.age -= 1;
        }

        return false;
    }
}

fn star_one(lines: &Vec<String>) -> usize {
    let ages: Vec<usize> = lines[0]
        .split(',')
        .map(|x| x.parse::<usize>().expect("Invalid number"))
        .collect();

    let mut fishes: Vec<LanternFish> = Vec::new();
    for age in ages {
        fishes.push(LanternFish::new(age));
    }

    for _ in 0..80 {
        let mut new_fishes: Vec<LanternFish> = Vec::new();
        for fish in &mut fishes {
            if fish.day_passes() {
                new_fishes.push(LanternFish::new(8));
            }
        }
        fishes.append(&mut new_fishes);
    }

    fishes.len()
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
    static TEST_DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 5934);
    }
}
