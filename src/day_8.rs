use fancy_regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct Mapping<'a> {
    pub start: &'a str,
    pub left: &'a str,
    pub right: &'a str,
}

pub fn parse_map<'a>(line: &'a str, re: &'a Regex) -> Mapping<'a> {
    let groups = re.captures(line).unwrap().unwrap();
    let start = groups.name("start").unwrap().as_str();
    let left = groups.name("left").unwrap().as_str();
    let right = groups.name("right").unwrap().as_str();
    Mapping { start, left, right }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::download_day;

    use super::parse_map;
    use fancy_regex::Regex;
    use num::integer::lcm;
    #[test]
    fn part_one() {
        let regex = Regex::new(r"(?P<start>.{3}) = \((?P<left>.{3}), (?P<right>.{3})\)").unwrap();
        let content = download_day(2023, 8);
        let mut line_iter = content.lines();
        let directions = line_iter.next().unwrap();
        line_iter.next();
        let mut maps = HashMap::new();
        for line in line_iter {
            let mapping = parse_map(line, &regex);
            maps.insert(mapping.start, mapping);
        }
        let mut current = "AAA";
        let mut count = 0;
        let mut directions_iter = directions.chars().cycle();
        while current != "ZZZ" {
            match directions_iter.next().unwrap() {
                'L' => current = maps[&current].left,
                'R' => current = maps[&current].right,
                _ => panic!("Unexpected Character"),
            }
            count += 1;
        }
        println!("Part One: {count}");
    }

    #[test]
    fn part_two() {
        let regex = Regex::new(r"(?P<start>.{3}) = \((?P<left>.{3}), (?P<right>.{3})\)").unwrap();
        let content = download_day(2023, 8);
        let mut line_iter = content.lines();
        let directions = line_iter.next().unwrap();
        line_iter.next();
        let mut maps = HashMap::new();
        let mut starting: Vec<&str> = vec![];
        for line in line_iter {
            let mapping = parse_map(line, &regex);
            maps.insert(mapping.start, mapping);
            if mapping.start.ends_with("A") {
                starting.push(mapping.start);
            }
        }
        let mut counts = vec![];
        let mut directions_iter = directions.chars().cycle();
        for mut current in starting {
            let mut count: usize = 0;
            while !current.ends_with("Z") {
                match directions_iter.next().unwrap() {
                    'L' => {
                        current = maps[&current].left;
                    }
                    'R' => {
                        current = maps[&current].right;
                    }
                    _ => panic!("Unexpected Character"),
                }
                count += 1;
            }
            counts.push(count);
        }
        let res = counts.iter().fold(1, |acc, x| lcm(acc, *x));
        println!("Part Two: {res}");
    }

    const SAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part_one_sample() {
        let regex = Regex::new(r"(?P<start>.{3}) = \((?P<left>.{3}), (?P<right>.{3})\)").unwrap();
        let content = SAMPLE;
        let mut line_iter = content.lines();
        let directions = line_iter.next().unwrap();
        line_iter.next();
        let mut maps = HashMap::new();
        for line in line_iter {
            let mapping = parse_map(line, &regex);
            maps.insert(mapping.start, mapping);
        }
        let mut current = "AAA";
        let mut count = 0;
        let mut directions_iter = directions.chars().cycle();
        while current != "ZZZ" {
            match directions_iter.next().unwrap() {
                'L' => current = maps[&current].left,
                'R' => current = maps[&current].right,
                _ => panic!("Unexpected Character"),
            }
            count += 1;
        }
        assert_eq!(count, 2)
    }
    const SAMPLE_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part_two_sample() {
        let regex = Regex::new(r"(?P<start>.{3}) = \((?P<left>.{3}), (?P<right>.{3})\)").unwrap();
        let content = SAMPLE_2;
        let mut line_iter = content.lines();
        let directions = line_iter.next().unwrap();
        line_iter.next();
        let mut maps = HashMap::new();
        let mut starting: Vec<&str> = vec![];
        for line in line_iter {
            let mapping = parse_map(line, &regex);
            maps.insert(mapping.start, mapping);
            if mapping.start.ends_with("A") {
                starting.push(mapping.start);
            }
        }
        let mut counts = vec![];
        let mut directions_iter = directions.chars().cycle();
        for mut current in starting {
            let mut count: usize = 0;
            while !current.ends_with("Z") {
                match directions_iter.next().unwrap() {
                    'L' => {
                        current = maps[&current].left;
                    }
                    'R' => {
                        current = maps[&current].right;
                    }
                    _ => panic!("Unexpected Character"),
                }
                count += 1;
            }
            counts.push(count);
        }
        let res = counts.iter().fold(1, |acc, x| lcm(acc, *x));
        println!("Part Two: {res}");
    }
}
