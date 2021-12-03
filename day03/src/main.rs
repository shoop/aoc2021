use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(lines: &[String], nrbits: usize) -> usize {
    let total = lines.iter().count();
    let mut bitcounts: Vec<usize> = vec![0; nrbits];

    for line in lines.iter() {
        for bit in 0..nrbits {
            if line.chars().nth(nrbits - bit - 1)
              .expect("Invalid line") == '1' {
                bitcounts[bit] += 1;
            }
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for bit in 0..nrbits {
        if bitcounts[bit] > (total >> 1) {
            gamma += 1 << bit;
        } else {
            epsilon += 1 << bit;
        }
    }

    gamma * epsilon
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines, 12);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines, 5);
        assert_eq!(ans, 198);
    }
}
