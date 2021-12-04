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

fn find_common_bit(lines: &[&str], bit_nr: usize, most_common: bool) -> char {
    let total = lines.iter().count();
    let mut count_ones = 0;
    for line in lines.iter() {
        if line.chars().nth(bit_nr)
            .expect("Invalid line") == '1' {
            count_ones += 1;
        }
    }

    if (total - count_ones) <= count_ones {
        return if most_common { '1' } else { '0' };
    } else {
        return if most_common { '0' } else { '1' };
    }
}

fn star_two(lines: &[String], nrbits: usize) -> usize {
    let mut lines_left: Vec<&str> = lines.iter().map(|x| x.as_str()).collect();
    let mut new_lines: Vec<&str> = Vec::with_capacity(lines.iter().count());

    // oxygen
    for bit in 0..nrbits {
        let common_bit = find_common_bit(&lines_left[..], bit, true);
        for line in lines_left.iter() {
            if line.chars().nth(bit).expect("Invalid line") == common_bit {
                new_lines.push(line);
            }
        }
        if new_lines.iter().count() == 1 {
            break;
        }
        lines_left = new_lines;
        new_lines = Vec::with_capacity(lines_left.iter().count());
    }
    let oxygen = usize::from_str_radix(new_lines[0], 2).expect("Invalid line");

    // co2
    let mut lines_left: Vec<&str> = lines.iter().map(|x| x.as_str()).collect();
    let mut new_lines: Vec<&str> = Vec::with_capacity(lines.iter().count());
    for bit in 0..nrbits {
        let common_bit = find_common_bit(&lines_left[..], bit, false);
        for line in lines_left.iter() {
            if line.chars().nth(bit).expect("Invalid line") == common_bit {
                new_lines.push(line);
            }
        }
        if new_lines.iter().count() == 1 {
            break;
        }
        lines_left = new_lines;
        new_lines = Vec::with_capacity(lines_left.iter().count());
    }
    let co2 = usize::from_str_radix(new_lines[0], 2).expect("Invalid line");

    oxygen * co2
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines, 12);
    println!("Star one: {}", ans);

    let ans = star_two(&lines, 12);
    println!("Star two: {}", ans);
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

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_two(&lines, 5);
        assert_eq!(ans, 230);
    }
}
