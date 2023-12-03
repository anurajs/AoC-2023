use std::{collections::HashMap, vec};

pub fn get_gears(content: &str) -> Vec<(usize, usize)> {
    let adjacency: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
    ];
    let mut grid: Vec<Vec<char>> = vec![];
    for line in content.lines() {
        grid.push(line.chars().collect());
    }
    let mut gear_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (i, line) in grid.iter().enumerate() {
        let mut c_iter = line.iter().enumerate();
        let mut buffer: String = String::from("");
        let mut gear = None;
        while let Some((j, c)) = c_iter.next() {
            if c.is_numeric() {
                buffer.push(c.clone());
                for (x, y) in adjacency {
                    if !(x + (i as i32) < 0
                        || y + (j as i32) < 0
                        || x + (i as i32) >= grid.len() as i32
                        || y + (j as i32) >= line.len() as i32)
                    {
                        let comp = grid[(x + i as i32) as usize][(y + j as i32) as usize];
                        if comp == '*' {
                            gear = Some(((x + i as i32) as usize, (y + j as i32) as usize));
                        }
                    }
                }
            } else {
                if gear.is_some() && buffer != "" {
                    let point = gear.unwrap();
                    gear_map
                        .entry(point)
                        .or_insert(Vec::new())
                        .push(buffer.parse().unwrap());
                }
                buffer.clear();
                gear = None;
            }
        }
        if gear.is_some() && buffer != "" {
            let point = gear.unwrap();
            gear_map
                .entry(point)
                .or_insert(Vec::new())
                .push(buffer.parse().unwrap());
        }
    }

    gear_map
        .into_iter()
        .filter(|(_, vec)| vec.len() == 2)
        .map(|(_, vec)| (vec[0], vec[1]))
        .collect()
}

pub fn get_part_numbers(content: &str) -> Vec<usize> {
    let adjacency: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
    ];
    let mut grid: Vec<Vec<char>> = vec![];
    for line in content.lines() {
        grid.push(line.chars().collect());
    }
    let mut adjacent = vec![];
    for (i, line) in grid.iter().enumerate() {
        let mut c_iter = line.iter().enumerate();
        let mut buffer: String = String::from("");
        let mut is_part = false;
        while let Some((j, c)) = c_iter.next() {
            if c.is_numeric() {
                buffer.push(c.clone());
                for (x, y) in adjacency {
                    if !(x + (i as i32) < 0
                        || y + (j as i32) < 0
                        || x + (i as i32) >= grid.len() as i32
                        || y + (j as i32) >= line.len() as i32)
                    {
                        let comp = grid[(x + i as i32) as usize][(y + j as i32) as usize];
                        if comp != '.' && !comp.is_numeric() {
                            is_part = true;
                        }
                    }
                }
            } else {
                if buffer != "" && is_part {
                    adjacent.push(buffer.parse().unwrap());
                }
                buffer.clear();
                is_part = false;
            }
        }
        if buffer != "" && is_part {
            adjacent.push(buffer.parse().unwrap());
        }
    }

    adjacent
}

#[cfg(test)]
mod tests {
    use crate::{
        day_3::{get_gears, get_part_numbers},
        download_day,
    };

    #[test]
    fn part_2() {
        let content = download_day(2023, 3);
        let gears = get_gears(&content);
        let result = gears.iter().map(|(a, b)| a * b).sum::<usize>();
        println!("{result}");
    }

    #[test]
    fn part_1() {
        let content = download_day(2023, 3);
        let result = get_part_numbers(&content);
        println!("part 1: {}", result.iter().sum::<usize>());
    }
    #[test]
    fn sample_input() {
        let result = get_part_numbers(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result.iter().sum::<usize>(), 4361);
    }
    #[test]
    fn sample_input_two() {
        let result = get_gears(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result.iter().map(|(a, b)| a * b).sum::<usize>(), 467835);
    }
}
