use std::collections::HashSet;

type Point = (isize, isize);
#[derive(Clone, Copy, Debug)]
pub enum MapUnit {
    VerticalSplitter,
    HorizontalSplitter,
    RightMirror,
    LeftMirror,
    Empty,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn movement(&self) -> Point {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Beam {
    location: Point,
    direction: Direction,
}

pub fn parse_map(content: &str) -> Vec<Vec<MapUnit>> {
    let mut res = vec![];
    let mut row = vec![];
    for line in content.lines() {
        for c in line.chars() {
            match c {
                '|' => row.push(MapUnit::VerticalSplitter),
                '-' => row.push(MapUnit::HorizontalSplitter),
                '/' => row.push(MapUnit::RightMirror),
                '\\' => row.push(MapUnit::LeftMirror),
                '.' => row.push(MapUnit::Empty),
                _ => panic!("Unexpected Character {c}"),
            }
        }
        res.push(row.clone());
        row.clear();
    }
    res
}

pub fn simulate_beams(map: &[Vec<MapUnit>], mut beams: Vec<Beam>) -> usize {
    let mut new_beams: Vec<Beam> = vec![];
    let mut visited: HashSet<Point> = HashSet::new();
    let mut unique_beam = HashSet::new();
    while !beams.is_empty() {
        for beam in beams.iter_mut() {
            unique_beam.insert(beam.clone());
            visited.insert(beam.location);
            let new_loc = (
                beam.location.0 + beam.direction.movement().0,
                beam.location.1 + beam.direction.movement().1,
            );
            if !(new_loc.0 < 0
                || new_loc.0 >= map[0].len() as isize
                || new_loc.1 < 0
                || new_loc.1 >= map.len() as isize)
            {
                beam.location = new_loc;
                match map[beam.location.1 as usize][beam.location.0 as usize] {
                    MapUnit::Empty => {}
                    MapUnit::RightMirror => match beam.direction {
                        Direction::North => beam.direction = Direction::East,
                        Direction::South => beam.direction = Direction::West,
                        Direction::East => beam.direction = Direction::North,
                        Direction::West => beam.direction = Direction::South,
                    },
                    MapUnit::LeftMirror => match beam.direction {
                        Direction::North => beam.direction = Direction::West,
                        Direction::South => beam.direction = Direction::East,
                        Direction::East => beam.direction = Direction::South,
                        Direction::West => beam.direction = Direction::North,
                    },
                    MapUnit::VerticalSplitter => match beam.direction {
                        Direction::North | Direction::South => {}
                        Direction::East | Direction::West => {
                            beam.direction = Direction::North;
                            let split = Beam {
                                location: beam.location,
                                direction: Direction::South,
                            };
                            if let None = unique_beam.get(&split) {
                                new_beams.push(split);
                            }
                        }
                    },
                    MapUnit::HorizontalSplitter => match beam.direction {
                        Direction::East | Direction::West => {}
                        Direction::North | Direction::South => {
                            beam.direction = Direction::East;
                            let split = Beam {
                                location: beam.location,
                                direction: Direction::West,
                            };
                            if let None = unique_beam.get(&split) {
                                new_beams.push(split);
                            }
                        }
                    },
                }
                if let None = unique_beam.get(&beam) {
                    new_beams.push(beam.clone())
                };
            }
        }
        std::mem::swap(&mut beams, &mut new_beams);
        new_beams.clear();
    }
    visited.len() - 1
}

#[cfg(test)]
mod tests {
    use crate::download_day;

    use super::{parse_map, simulate_beams, Beam, Direction};

    #[test]
    fn part_one() {
        let content = download_day(2023, 16);
        let map = parse_map(&content);
        let res = simulate_beams(
            &map,
            vec![Beam {
                location: (-1, 0),
                direction: Direction::East,
            }],
        );
        println!("Part One: {res}");
    }
    #[test]
    fn part_two() {
        let content = download_day(2023, 16);
        let map = parse_map(&content);
        let mut beams = vec![];
        beams.append(
            &mut (0..map.len() as isize)
                .map(|x| Beam {
                    location: (-1, x),
                    direction: Direction::East,
                })
                .collect(),
        );
        beams.append(
            &mut (0..map.len() as isize)
                .map(|x| Beam {
                    location: (map[0].len() as isize, x),
                    direction: Direction::West,
                })
                .collect(),
        );
        beams.append(
            &mut (0..map[0].len() as isize)
                .map(|x| Beam {
                    location: (x, -1),
                    direction: Direction::South,
                })
                .collect(),
        );
        beams.append(
            &mut (0..map[0].len() as isize)
                .map(|x| Beam {
                    location: (x, map.len() as isize),
                    direction: Direction::South,
                })
                .collect(),
        );
        let res = beams
            .iter()
            .map(|b| simulate_beams(&map, vec![b.clone()]))
            .max()
            .unwrap();
        println!("Part Two: {res}");
    }

    const SAMPLE: &str = include_str!("../day16sample.txt");

    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let map = parse_map(content);
        let res = simulate_beams(
            &map,
            vec![Beam {
                location: (-1, 0),
                direction: Direction::East,
            }],
        );
        assert_eq!(res, 46);
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE;
        let map = parse_map(content);
        let mut beams = vec![];
        beams.append(
            &mut (0..map.len() as isize)
                .map(|x| Beam {
                    location: (-1, x),
                    direction: Direction::East,
                })
                .collect(),
        );
        beams.append(
            &mut (0..map.len() as isize)
                .map(|x| Beam {
                    location: (map[0].len() as isize, x),
                    direction: Direction::West,
                })
                .collect(),
        );
        beams.append(
            &mut (0..map[0].len() as isize)
                .map(|x| Beam {
                    location: (x, -1),
                    direction: Direction::South,
                })
                .collect(),
        );
        beams.append(
            &mut (0..map[0].len() as isize)
                .map(|x| Beam {
                    location: (x, map.len() as isize),
                    direction: Direction::South,
                })
                .collect(),
        );
        let res = beams
            .iter()
            .map(|b| simulate_beams(&map, vec![b.clone()]))
            .max()
            .unwrap();
        assert_eq!(res, 51);
    }
}
