pub type Point = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Upright,
    Upleft,
    Down,
    Downright,
    Downleft,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct DigInstruction {
    direction: Direction,
    amount: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DugEdge {
    point: Point,
    direction: Direction,
}

pub fn parse_plan(content: &str) -> Vec<DigInstruction> {
    let mut res = vec![];
    for line in content.lines() {
        let mut line_iter = line.split_whitespace();
        let direction = match line_iter.next().unwrap().chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unexpected direction"),
        };
        let amount: usize = line_iter.next().unwrap().parse().unwrap();
        res.push(DigInstruction { direction, amount })
    }
    res
}

pub fn parse_hex_plan(content: &str) -> Vec<DigInstruction> {
    let mut res = vec![];
    for line in content.lines() {
        let hex = &line.split_whitespace().nth(2).unwrap()[2..8];
        let amount = usize::from_str_radix(&hex[0..5], 16).unwrap();
        let direction = match &hex[5..] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("unexpected character"),
        };
        res.push(DigInstruction { direction, amount })
    }
    res
}

pub fn find_area(instructions: &[DigInstruction]) -> isize {
    let mut current_row = 0;
    let mut current_column = 0;
    let mut edges: Vec<DugEdge> = vec![];
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                if let Some(prev) = edges.last_mut() {
                    match prev.direction {
                        Direction::Left => prev.direction = Direction::Upright,
                        Direction::Right => prev.direction = Direction::Upleft,
                        _ => {}
                    }
                }
                for i in 1..=instruction.amount as isize {
                    edges.push(DugEdge {
                        point: (current_column - i, current_row),
                        direction: instruction.direction,
                    })
                }
                current_column -= instruction.amount as isize;
            }
            Direction::Down => {
                if let Some(prev) = edges.last_mut() {
                    match prev.direction {
                        Direction::Left => prev.direction = Direction::Downleft,
                        Direction::Right => prev.direction = Direction::Downright,
                        _ => {}
                    }
                }
                for i in 1..=instruction.amount as isize {
                    edges.push(DugEdge {
                        point: (current_column + i, current_row),
                        direction: instruction.direction,
                    })
                }
                current_column += instruction.amount as isize;
            }
            Direction::Left => {
                if let Some(prev) = edges.last_mut() {
                    match prev.direction {
                        Direction::Up => prev.direction = Direction::Downleft,
                        Direction::Down => prev.direction = Direction::Upleft,
                        _ => {}
                    }
                }
                for i in 1..=instruction.amount as isize {
                    edges.push(DugEdge {
                        point: (current_column, current_row - i),
                        direction: instruction.direction,
                    })
                }
                current_row -= instruction.amount as isize;
            }
            Direction::Right => {
                if let Some(prev) = edges.last_mut() {
                    match prev.direction {
                        Direction::Up => prev.direction = Direction::Downright,
                        Direction::Down => prev.direction = Direction::Upright,
                        _ => {}
                    }
                }
                for i in 1..=instruction.amount as isize {
                    edges.push(DugEdge {
                        point: (current_column, current_row + i),
                        direction: instruction.direction,
                    })
                }
                current_row += instruction.amount as isize;
            }
            _ => {}
        }
    }

    if let Some(prev) = edges.last_mut() {
        prev.direction = Direction::Downright
    }
    edges.sort();
    let mut edge_iter = edges.iter().peekable();
    let mut total = 0;
    let mut inside = false;

    while let Some(current) = edge_iter.next() {
        match current.direction {
            Direction::Up | Direction::Down | Direction::Upleft | Direction::Upright => {
                inside = !inside
            }
            _ => {}
        };
        if let Some(next) = edge_iter.peek() {
            if current.point.0 == next.point.0 {
                if inside {
                    let diff = (current.point.1 - next.point.1).abs() - 1;
                    total += diff;
                }
            } else {
                inside = false;
            }
        }
    }
    edges.len() as isize + total
}

#[cfg(test)]
mod tests {
    use crate::{day_18::find_area, download_day};

    use super::{parse_hex_plan, parse_plan};

    #[test]
    fn part_one() {
        let content = download_day(2023, 18);
        let plan = parse_plan(&content);
        let res = find_area(&plan);
        println!("Part One: {res}");
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 18);
        let plan = parse_hex_plan(&content);
        let res = find_area(&plan);
        println!("Part One: {res}");
    }

    const SAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let instructions = parse_plan(content);
        let res = find_area(&instructions);
        assert_eq!(res, 62)
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE;
        let instructions = parse_hex_plan(content);
        let res = find_area(&instructions);
        assert_eq!(res, 952408144115)
    }
}
