use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(lines: Vec<String>) -> isize {
    let mut prev: isize = -1;
    let mut result: isize = 0;

    for line in lines.iter() {
        let cur = line.parse::<isize>()
            .expect(&format!("Invalid number: {}", &line));
        if prev != -1 && cur > prev {
            result += 1;
        }
        prev = cur;
    }

    result
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(lines);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(lines);
        assert_eq!(ans, 7);
    }
}
