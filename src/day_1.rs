use fancy_regex::Regex;
use std::collections::HashMap;

pub fn calculate_line_one(line: &str) -> usize {
    let mut first = None;
    let mut second = None;
    for c in line.chars() {
        if c.is_numeric() {
            match (first, second) {
                (None, _) => {
                    first = Some(c);
                    second = Some(c);
                }
                (_, _) => {
                    second = Some(c);
                }
            }
        }
    }
    let combined = format!(
        "{}{}",
        first.expect("expected a first digit"),
        second.expect("expected a second digit")
    );

    combined
        .parse()
        .expect("expected both digits to be numbers")
}

pub fn calculate_line_two(line: &str, digits: &HashMap<&str, usize>) -> usize {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut first = None;
    let mut second = None;
    for (digit, val) in digits {
        if let Some(idx) = line.find(digit) {
            if (idx as i32) < min {
                first = Some(*val);
                min = idx as i32;
            }
        } else {
            continue;
        }

        if let Some(idx) = line.rfind(digit) {
            if (idx as i32) > max {
                second = Some(*val);
                max = idx as i32;
            }
        }
    }
    let combined = format!(
        "{}{}",
        first.expect("expected a first digit"),
        second.expect("expected a second digit")
    );

    combined
        .parse()
        .expect("expected both digits to be numbers")
}

pub fn calculate_line_two_regex(line: &str, regex: &Regex, digits: &HashMap<&str, usize>) -> usize {
    let matches: Vec<_> = regex.captures_iter(line).collect();
    let first = &matches[0].as_ref().unwrap()[1];
    let second = &matches[matches.len() - 1].as_ref().unwrap()[1];

    let combined = format!(
        "{}{}",
        digits.get(first).expect("digit should be in map"),
        digits.get(second).expect("digit should be in map")
    );

    return combined
        .parse()
        .expect("expected both digits to be numbers");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fancy_regex::Regex;
    use once_cell::sync::Lazy;

    use super::{calculate_line_one, calculate_line_two, calculate_line_two_regex};
    use crate::download_day;

    static DIGITS: Lazy<HashMap<&str, usize>> = Lazy::new(|| {
        HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ])
    });

    #[test]
    fn part_1() {
        let content = download_day(2023, 1);
        let total: usize = content.lines().map(calculate_line_one).sum();
        println!("Part 1: {total}");
    }

    #[test]
    fn part_2() {
        let content = download_day(2023, 1);
        let total: usize = content
            .lines()
            .map(|line| calculate_line_two(line, &DIGITS))
            .sum();

        println!("Part two: {total}");
    }

    #[test]
    fn part_2_regex() {
        let regex =
            Regex::new(r"(?m)(?=([1-9]|one|two|three|four|five|six|seven|eight|nine))").unwrap();
        let content = download_day(2023, 1);
        let total: usize = content
            .lines()
            .map(|line| calculate_line_two_regex(line, &regex, &DIGITS))
            .sum();

        println!("Part two: {total}");
    }

    #[test]
    fn calculate_line_two_test() {
        let digits: HashMap<&str, usize> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);

        let line = "two1nine";
        assert_eq!(calculate_line_two(line, &digits), 29);

        let line = "eightwothree";
        assert_eq!(calculate_line_two(line, &digits), 83);

        let line = "abcone2threexyz";
        assert_eq!(calculate_line_two(line, &digits), 13);

        let line = "xtwone3four";
        assert_eq!(calculate_line_two(line, &digits), 24);

        let line = "4nineeightseven2";
        assert_eq!(calculate_line_two(line, &digits), 42);

        let line = "zoneight234";
        assert_eq!(calculate_line_two(line, &digits), 14);

        let line = "7pqrstsixteen";
        assert_eq!(calculate_line_two(line, &digits), 76);
    }

    #[test]
    fn calculate_line_one_test() {
        let mut line = "1abc2";
        assert_eq!(calculate_line_one(line), 12);

        line = "pqr3stu8vwx";
        assert_eq!(calculate_line_one(line), 38);

        line = "a1b2c3d4e5f";
        assert_eq!(calculate_line_one(line), 15);

        line = "treb7uchet";
        assert_eq!(calculate_line_one(line), 77);
    }
}
