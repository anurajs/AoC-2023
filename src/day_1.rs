pub fn calculate_line(line: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::{day_1::calculate_line, download_day};

    #[test]
    fn part_1() {
        let content = download_day(2023, 1);
        let mut total = 0;
        for line in content.lines() {
            total += calculate_line(line);
        }
        println!("Part 1: {total}");
    }

    #[test]
    fn calculate_line_test() {
        let mut line = "1abc2";
        assert_eq!(calculate_line(line), 12);

        line = "pqr3stu8vwx";
        assert_eq!(calculate_line(line), 38);

        line = "a1b2c3d4e5f";
        assert_eq!(calculate_line(line), 15);

        line = "treb7uchet";
        assert_eq!(calculate_line(line), 77);
    }
}
