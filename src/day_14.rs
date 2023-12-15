use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapUnit {
    Round,
    Cube,
    Empty,
}

impl Display for MapUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MapUnit::Round => write!(f, "O"),
            MapUnit::Cube => write!(f, "#"),
            MapUnit::Empty => write!(f, "."),
        }
    }
}

pub fn tilt_north(map: &[Vec<MapUnit>]) -> (usize, Vec<Vec<MapUnit>>) {
    let mut total_load = 0;
    let mut new_map = vec![vec![MapUnit::Empty; map[0].len()]; map.len()];
    for i in 0..map[0].len() {
        let mut next_available = 0;
        for j in 0..map.len() {
            match map[j][i] {
                MapUnit::Round => {
                    total_load += map.len() - next_available;
                    new_map[next_available][i] = MapUnit::Round;
                    next_available = next_available + 1;
                }
                MapUnit::Cube => {
                    next_available = j + 1;
                    new_map[j][i] = MapUnit::Cube;
                }
                MapUnit::Empty => {}
            }
        }
    }

    (total_load, new_map)
}
pub fn tilt_east(map: &[Vec<MapUnit>]) -> (usize, Vec<Vec<MapUnit>>) {
    let mut total_load = 0;
    let mut new_map = vec![vec![MapUnit::Empty; map[0].len()]; map.len()];
    for i in 0..map.len() {
        let mut next_available = map[0].len() as isize - 1;
        for j in (0..map[0].len()).rev() {
            match map[i][j] {
                MapUnit::Round => {
                    total_load += map.len() - i;
                    new_map[i][next_available as usize] = MapUnit::Round;
                    next_available = next_available - 1;
                }
                MapUnit::Cube => {
                    next_available = j as isize - 1;
                    new_map[i][j] = MapUnit::Cube;
                }
                MapUnit::Empty => {}
            }
        }
    }

    (total_load, new_map)
}
pub fn tilt_south(map: &[Vec<MapUnit>]) -> (usize, Vec<Vec<MapUnit>>) {
    let mut total_load = 0;
    let mut new_map = vec![vec![MapUnit::Empty; map[0].len()]; map.len()];
    for i in 0..map[0].len() {
        let mut next_available = map.len() as isize - 1;
        for j in (0..map.len()).rev() {
            match map[j][i] {
                MapUnit::Round => {
                    total_load += map.len() - next_available as usize;
                    new_map[next_available as usize][i] = MapUnit::Round;
                    next_available = next_available - 1;
                }
                MapUnit::Cube => {
                    next_available = j as isize - 1;
                    new_map[j][i] = MapUnit::Cube;
                }
                MapUnit::Empty => {}
            }
        }
    }

    (total_load, new_map)
}
pub fn tilt_west(map: &[Vec<MapUnit>]) -> (usize, Vec<Vec<MapUnit>>) {
    let mut total_load = 0;
    let mut new_map = vec![vec![MapUnit::Empty; map[0].len()]; map.len()];
    for i in 0..map.len() {
        let mut next_available = 0;
        for j in 0..map[0].len() {
            match map[i][j] {
                MapUnit::Round => {
                    total_load += map.len() - i;
                    new_map[i][next_available] = MapUnit::Round;
                    next_available = next_available + 1;
                }
                MapUnit::Cube => {
                    next_available = j + 1;
                    new_map[i][j] = MapUnit::Cube;
                }
                MapUnit::Empty => {}
            }
        }
    }

    (total_load, new_map)
}

pub fn do_cycle(map: &[Vec<MapUnit>]) -> (usize, Vec<Vec<MapUnit>>) {
    let (_, map) = tilt_north(map);
    let (_, map) = tilt_west(&map);
    let (_, map) = tilt_south(&map);
    let (score, map) = tilt_east(&map);

    (score, map)
}
pub fn calculate_load(map: &[Vec<MapUnit>]) -> usize {
    let mut load = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if MapUnit::Round == map[i][j] {
                load += map.len() - i;
            }
        }
    }

    load
}

pub fn parse_map(content: &str) -> Vec<Vec<MapUnit>> {
    let mut map = vec![];
    let mut row = vec![];
    for line in content.lines() {
        for c in line.chars() {
            match c {
                'O' => row.push(MapUnit::Round),
                '#' => row.push(MapUnit::Cube),
                '.' => row.push(MapUnit::Empty),
                _ => panic!("Unexpected character"),
            }
        }
        map.push(row.clone());
        row.clear()
    }

    map
}

#[cfg(test)]
mod tests {

    use crate::{
        day_14::{calculate_load, MapUnit},
        download_day,
    };

    use super::{do_cycle, parse_map, tilt_north};

    #[test]
    fn part_one() {
        let content = download_day(2023, 14);
        let map = parse_map(&content[..]);
        let (res, _) = tilt_north(&map);
        println!("Part One: {res}");
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 14);
        let mut map = parse_map(&content[..]);
        let s;
        let mut map_vec: Vec<Vec<Vec<MapUnit>>> = vec![map.clone()];
        loop {
            (_, map) = do_cycle(&map);
            if let Some(cycle_start) = map_vec.iter().position(|x| x == &map) {
                let cycle_length = map_vec.len() - cycle_start;
                let offset = 1000000000 - cycle_start;
                let pos = offset % cycle_length;
                s = calculate_load(&map_vec[cycle_start..][pos]);
                break;
            } else {
                map_vec.push(map.clone());
            }
        }
        println!("Part Two: {s}");
    }

    const SAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const SAMPLE_2: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let map = parse_map(content);
        let (res, _) = tilt_north(&map);
        assert_eq!(res, 136);
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE;
        let mut map = parse_map(content);
        let s;
        let mut map_vec: Vec<Vec<Vec<MapUnit>>> = vec![];
        loop {
            (_, map) = do_cycle(&map);
            if let Some(idx) = map_vec.iter().position(|x| x == &map) {
                let cycle_start = idx;
                let cycle_length = map_vec.len() - cycle_start;
                let offset = 1000000000 - cycle_start - 1;
                let pos = offset % cycle_length;
                let map_vec = &map_vec[cycle_start..];
                s = calculate_load(&map_vec[pos]);
                break;
            } else {
                map_vec.push(map.clone());
            }
        }
        assert_eq!(s, 64)
    }

    #[test]
    fn debugging() {
        let m = parse_map(SAMPLE_2);
        let res = calculate_load(&m);
        println!("{res}");
    }
}
