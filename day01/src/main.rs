use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(lines: &Vec<String>) -> isize {
    let mut prev: isize = -1;
    let mut result: isize = 0;

    for line in lines.iter() {
        let cur = line
            .parse::<isize>()
            .expect(&format!("Invalid number: {}", &line));
        if prev != -1 && cur > prev {
            result += 1;
        }
        prev = cur;
    }

    result
}

fn star_two(lines: &Vec<String>) -> usize {
    lines.into_iter()
        .map(|line| {
            let cur = line
            .parse::<isize>()
            .expect(&format!("Invalid number: {}", &line));
            cur
        })
        .collect::<Vec<isize>>()
        .windows(4)
        // Sliding window comparison with dropped terms: el[0] + el[1] + el[2] < el[1] + el[2] + el[3] <=> el[0] < el[3]
        .filter(|w| w[0] < w[3])
        .count()

    // let mut totals: [isize; 3] = [0; 3];
    // let mut result: isize = 0;

    // for (idx, line) in lines.into_iter().enumerate() {
    //     let cur = line
    //         .parse::<isize>()
    //         .expect(&format!("Invalid number: {}", &line));
    //     if idx < 3 {
    //         totals[0] += cur;
    //     }
    //     totals[1] += cur;
    //     if idx > 1 {
    //         totals[2] += cur;
    //     }
    //     if idx > 2 {
    //         if totals[1] > totals[0] {
    //             result += 1;
    //         }
    //         totals[0] = totals[1];
    //         totals[1] = totals[2];
    //         totals[2] = cur;
    //     }
    // }

    // result
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
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 7);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 5);
    }
}
