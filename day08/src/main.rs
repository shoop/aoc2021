use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn parse_display(alldigits: &str, currentdisplay: &str) -> usize {
    // Thanks to Bras (https://github.com/MBras) for the idea of the solution, comparing differences in wires

    let digit_strings: Vec<&str> = alldigits.split(' ').collect();

    let mut digits: Vec<HashSet<char>> = vec![HashSet::new(); 10];

    // Find the '1'
    digits[1] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 2)
        .unwrap()
        .chars());

    // Find the '7'
    digits[7] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 3)
        .unwrap()
        .chars());

    // Find the '4'
    digits[4] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 4)
        .unwrap()
        .chars());

    // Find the '8'
    digits[8] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 7)
        .unwrap()
        .chars());

    // Find the '3'
    // The set difference with the '7' should be 2 wires
    digits[3] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 5
            && HashSet::from_iter(s.clone().chars()).difference(&digits[7]).count() == 2)
        .unwrap()
        .chars());

    // Find the '6'
    // The set difference with the '7' should be 4 wires
    digits[6] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 6
            && HashSet::from_iter(s.clone().chars()).difference(&digits[7]).count() == 4)
        .unwrap()
        .chars());

    // Find the '0'
    // The set difference with the '3' should be 2 wires
    digits[0] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 6
            && !HashSet::from_iter(s.clone().chars()).eq(&digits[6])
            && HashSet::from_iter(s.clone().chars()).difference(&digits[3]).count() == 2)
        .unwrap()
        .chars());

    // Find the '9'
    // The last one remaining with 6 wires
    digits[9] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 6
            && !HashSet::from_iter(s.clone().chars()).eq(&digits[6])
            && !HashSet::from_iter(s.clone().chars()).eq(&digits[0]))
        .unwrap()
        .chars());

    // Find the '5'
    // The difference with the four is 2 wires
    digits[5] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 5
            && !HashSet::from_iter(s.clone().chars()).eq(&digits[3])
            && HashSet::from_iter(s.clone().chars()).difference(&digits[4]).count() == 2)
        .unwrap()
        .chars());

    // Find the '2'
    // The last one remaining with 5 wires
    digits[2] = HashSet::from_iter(digit_strings
        .iter()
        .find(|s| s.len() == 5
            && !HashSet::from_iter(s.clone().chars()).eq(&digits[3])
            && !HashSet::from_iter(s.clone().chars()).eq(&digits[5]))
        .unwrap()
        .chars());


    // Now work out the display
    let mut tot = 0;
    for (i, d) in currentdisplay.split(' ').rev().enumerate() {
        let set: HashSet<char> = HashSet::from_iter(d.chars());
        let pos = digits.iter().enumerate().find(|(_, s)| (*s).eq(&set)).unwrap().0;
        tot += usize::pow(10, i as u32) * pos;
    }

    tot
}

fn star_two(lines: &Vec<String>) -> usize {
    let mut count: usize = 0;
    for line in lines {
        let mut split = line.split(" | ");
        let alldigits = split.next().unwrap();
        let currentdisplay = split.next().unwrap();
        count += parse_display(alldigits, currentdisplay);
    }

    count
}

fn star_one(lines: &Vec<String>) -> usize {
    let mut count = 0;
    for line in lines {
        let mut split = line.split(" | ");
        let _ = split.next().unwrap();
        let currentdisplay = split.next().unwrap();

        for digit in currentdisplay.split(' ') {
            match digit.len() {
                2 => count += 1, // digit 1
                4 => count += 1, // digit 4
                3 => count += 1, // digit 7
                7 => count += 1, // digit 8
                _ => (),
            }
        }
    }

    count as usize
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
    static TEST_DATA: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 26);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 61229);
    }
}
