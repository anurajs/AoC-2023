use fancy_regex::Regex;

#[derive(PartialEq, Eq, Debug)]
pub struct CubeGame {
    red: usize,
    green: usize,
    blue: usize,
    id: usize,
}

pub fn cube_counter_regex(line: &str) -> CubeGame {
    let re = Regex::new(r"Game (\d+):").unwrap();
    let id: usize = re.captures(line).unwrap().expect("Expected Id")[1]
        .parse()
        .expect("Expected number");

    let (mut red, mut green, mut blue) = (0, 0, 0);
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    for m in re.captures_iter(line) {
        let m = m.unwrap();
        let amount: usize = m[1].parse().expect("Expected amount");
        let color = &m[2];
        match color {
            "red" => red = red.max(amount),
            "green" => green = green.max(amount),
            "blue" => blue = blue.max(amount),
            _ => {}
        }
    }

    CubeGame {
        red,
        green,
        blue,
        id,
    }
}

pub fn cube_counter(line: &str) -> CubeGame {
    let mut iter = line.split(" ").into_iter();
    let mut id: usize = 0;
    let mut red: usize = 0;
    let mut blue: usize = 0;
    let mut green: usize = 0;
    if let Some(_) = iter.next() {
        if let Some(id_string) = iter.next() {
            id = id_string[..id_string.len() - 1]
                .parse()
                .expect("Expected ID number");
        }
    }
    let mut iter = iter.peekable();
    while let Some(amount) = iter.next() {
        let n: usize = amount.parse().expect("Expected number");
        if let Some(mut color) = iter.next() {
            if let Some(_) = iter.peek() {
                color = &color[..color.len() - 1];
            }
            match color {
                "blue" => blue = blue.max(n),
                "green" => green = green.max(n),
                "red" => red = red.max(n),
                _ => {}
            }
        }
    }
    CubeGame {
        red,
        green,
        blue,
        id,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        day_2::{cube_counter, cube_counter_regex},
        download_day,
    };

    use super::CubeGame;

    #[test]
    fn part_1() {
        let content = download_day(2023, 2);
        let result: usize = content
            .lines()
            .map(cube_counter)
            .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
            .map(|game| game.id)
            .sum();
        println!("Part 1: {result}")
    }

    #[test]
    fn part_2() {
        let content = download_day(2023, 2);
        let result: usize = content
            .lines()
            .map(cube_counter)
            .map(|game| game.red * game.blue * game.green)
            .sum();

        println!("Part 2: {result}")
    }

    #[test]
    fn part_1_regex() {
        let content = download_day(2023, 2);
        let result: usize = content
            .lines()
            .map(cube_counter_regex)
            .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
            .map(|game| game.id)
            .sum();
        println!("Part 1: {result}")
    }

    #[test]
    fn part_2_regex() {
        let content = download_day(2023, 2);
        let result: usize = content
            .lines()
            .map(cube_counter_regex)
            .map(|game| game.red * game.blue * game.green)
            .sum();

        println!("Part 2: {result}")
    }

    #[test]
    fn cube_counter_test() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = CubeGame {
            id: 1,
            blue: 6,
            red: 4,
            green: 2,
        };
        assert_eq!(cube_counter(line), expected);
        assert_eq!(cube_counter_regex(line), expected);
    }

    #[test]
    fn multiply_test() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = cube_counter(line);
        let result = game.red * game.blue * game.green;
        assert_eq!(result, 1560)
    }
}
