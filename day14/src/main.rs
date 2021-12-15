use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Rule {
    new: char,
    count: usize,
}

fn parse(lines: &Vec<String>) -> (HashMap<(char, char), Rule>, HashMap<char, usize>) {
    let mut rules: HashMap<(char, char), Rule> = HashMap::new();

    let mut iter = lines.iter();
    let initial: String = iter.next().unwrap().to_string();
    iter.next().unwrap();

    while let Some(line) = iter.next() {
        let mut split = line.split(" -> ");
        let pair = split.next().unwrap();
        let new = split.next().unwrap();
        rules.insert((pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()), Rule { new: new.chars().nth(0).unwrap(), count: 0 });
    }

    for i in 0..initial.chars().count() - 1 {
        let left = initial.chars().nth(i).unwrap();
        let right = initial.chars().nth(i+1).unwrap();
        rules.get_mut(&(left, right)).unwrap().count += 1;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for ch in initial.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    (rules, counts)
}

fn run_gen(rules: &HashMap<(char, char), Rule>, counts: &mut HashMap<char, usize>) -> HashMap<(char, char), Rule> {
    let mut new_rules = rules.clone();
    for (_, rule) in new_rules.iter_mut() {
        rule.count = 0;
    }

    for ((left, right), rule) in rules {
        if rule.count == 0 {
            continue;
        }

        let leftrule = new_rules.get_mut(&(*left, rule.new)).unwrap();
        leftrule.count += rule.count;
        let rightrule = new_rules.get_mut(&(rule.new, *right)).unwrap();
        rightrule.count += rule.count;

        *counts.entry(rule.new).or_insert(0) += rule.count;
    }

    new_rules
}

fn run(lines: &Vec<String>, count: usize) -> usize {
    let parse_result = parse(lines);
    let mut rules = parse_result.0;
    let mut counts = parse_result.1;

    for _ in 0..count {
        rules = run_gen(&rules, &mut counts);
    }

    let max = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _)| k).unwrap();
    let min = counts.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(k, _)| k).unwrap();

    counts[max] - counts[min]
}

fn star_one(lines: &Vec<String>) -> usize {
    run(lines, 10)
}

fn star_two(lines: &Vec<String>) -> usize {
    run(lines, 40)
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
    static TEST_DATA: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 1588);
    }
}
