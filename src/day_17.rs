use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub type Point = (isize, isize);

const ALL_DIRECTIONS: [Option<Direction>; 4] = [
    Some(Direction::North),
    Some(Direction::East),
    Some(Direction::South),
    Some(Direction::West),
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    South,
    West,
    North,
    East,
}

impl Direction {
    pub fn movement(&self) -> Point {
        match self {
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::North => (0, -1),
            Self::East => (1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Visitable {
    distance: usize,
    location: Point,
    direction: Option<Direction>,
}

pub fn djikstras(map: &[Vec<usize>], min: usize, max: usize) -> usize {
    let mut visited: HashSet<(Point, Option<Direction>)> = HashSet::new();
    let mut to_visit: BinaryHeap<Reverse<Visitable>> = BinaryHeap::new();
    to_visit.push(Reverse(Visitable {
        location: (0, 0),
        direction: None,
        distance: 0,
    }));
    while let Some(Reverse(current)) = to_visit.pop() {
        if visited.contains(&(current.location, current.direction)) {
            continue;
        }
        if current.location == (map[0].len() as isize - 1, map.len() as isize - 1) {
            return current.distance;
        }
        visited.insert((current.location, current.direction));
        for (idx, dir) in ALL_DIRECTIONS.iter().enumerate() {
            if *dir == current.direction || ALL_DIRECTIONS[(idx + 2) % 4] == current.direction {
                continue;
            }
            let mut total = 0;
            for i in 1..min {
                let new_loc = (
                    current.location.0 + dir.unwrap().movement().0 * i as isize,
                    current.location.1 + dir.unwrap().movement().1 * i as isize,
                );
                if !(new_loc.0 < 0
                    || new_loc.0 >= map[0].len() as isize
                    || new_loc.1 < 0
                    || new_loc.1 >= map.len() as isize)
                {
                    total += map[new_loc.1 as usize][new_loc.0 as usize];
                }
            }
            for i in min..=max {
                let new_loc = (
                    current.location.0 + dir.unwrap().movement().0 * i as isize,
                    current.location.1 + dir.unwrap().movement().1 * i as isize,
                );
                if !(new_loc.0 < 0
                    || new_loc.0 >= map[0].len() as isize
                    || new_loc.1 < 0
                    || new_loc.1 >= map.len() as isize)
                {
                    total += map[new_loc.1 as usize][new_loc.0 as usize];
                    to_visit.push(Reverse(Visitable {
                        distance: current.distance + total,
                        location: new_loc,
                        direction: Some(dir.unwrap()),
                    }));
                }
            }
        }
    }

    0
}

pub fn parse_map(content: &str) -> Vec<Vec<usize>> {
    let mut res = vec![];
    let mut row = vec![];
    for line in content.lines() {
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize);
        }
        res.push(row.clone());
        row.clear();
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::download_day;

    use super::{djikstras, parse_map};

    #[test]
    fn part_one() {
        let content = download_day(2023, 17);
        let map = parse_map(&content);
        let res = djikstras(&map, 1, 3);
        println!("Part One: {res}");
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 17);
        let map = parse_map(&content);
        let res = djikstras(&map, 4, 10);
        println!("Part Two: {res}");
    }

    const SAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const SAMPLE_2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let map = parse_map(content);
        let res = djikstras(&map, 1, 3);
        assert_eq!(res, 102)
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE_2;
        let map = parse_map(content);
        let res = djikstras(&map, 4, 10);

        assert_eq!(res, 71);
    }
}
