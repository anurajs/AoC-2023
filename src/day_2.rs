#[derive(PartialEq, Eq, Debug)]
pub struct CubeGame {
    red: usize,
    green: usize,
    blue: usize,
    id: usize,
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
        red: red,
        green: green,
        blue: blue,
        id: id,
    }
}

#[cfg(test)]
mod tests {
    use crate::{day_2::cube_counter, download_day};

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
    fn cube_counter_test() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = CubeGame {
            id: 1,
            blue: 6,
            red: 4,
            green: 2,
        };
        assert_eq!(cube_counter(line), expected);
    }

    #[test]
    fn multiply_test() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = cube_counter(line);
        let result = game.red * game.blue * game.green;
        assert_eq!(result, 1560)
    }
}
