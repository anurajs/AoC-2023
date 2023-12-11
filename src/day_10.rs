use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

pub fn check_north(current: Point, map: &Vec<Vec<char>>) -> Option<Point> {
    if current.0 == 0 {
        return None;
    }

    let allowed = HashSet::from(['S', '|', 'J', 'L']);
    let receivers = HashSet::from(['|', '7', 'F']);

    if !allowed.contains(&map[current.0][current.1]) {
        return None;
    }
    if !receivers.contains(&map[current.0 - 1][current.1]) {
        return None;
    }

    return Some((current.0 - 1, current.1));
}

pub fn check_east(current: Point, map: &Vec<Vec<char>>) -> Option<Point> {
    if current.1 == map[0].len() {
        return None;
    }

    let allowed = HashSet::from(['S', '-', 'L', 'F']);
    let receivers = HashSet::from(['-', '7', 'J']);

    if !allowed.contains(&map[current.0][current.1]) {
        return None;
    }
    if !receivers.contains(&map[current.0][current.1 + 1]) {
        return None;
    }

    return Some((current.0, current.1 + 1));
}

pub fn check_west(current: Point, map: &Vec<Vec<char>>) -> Option<Point> {
    if current.1 == 0 {
        return None;
    }

    let allowed = HashSet::from(['S', '-', 'J', '7']);
    let receivers = HashSet::from(['-', 'L', 'F']);

    if !allowed.contains(&map[current.0][current.1]) {
        return None;
    }
    if !receivers.contains(&map[current.0][current.1 - 1]) {
        return None;
    }

    return Some((current.0, current.1 - 1));
}

pub fn check_south(current: Point, map: &Vec<Vec<char>>) -> Option<Point> {
    if current.0 == map.len() {
        return None;
    }

    let allowed = HashSet::from(['S', '|', '7', 'F']);
    let receivers = HashSet::from(['|', 'L', 'J']);

    if !allowed.contains(&map[current.0][current.1]) {
        return None;
    }
    if !receivers.contains(&map[current.0 + 1][current.1]) {
        return None;
    }

    return Some((current.0 + 1, current.1));
}

pub fn djikstras_furthest(starting: Point, map: Vec<Vec<char>>) -> (usize, HashSet<Point>) {
    let mut distance_tracker: HashMap<Point, usize> = HashMap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    distance_tracker.entry(starting).or_insert(0);

    let mut current = Some(starting);

    while let Some(cur) = current {
        visited.insert(cur);
        let d = *distance_tracker.get(&cur).unwrap();
        if let Some(p) = check_north(cur, &map) {
            distance_tracker
                .entry(p)
                .and_modify(|x| *x = *x.min(&mut (d + 1)))
                .or_insert(d + 1);
        }
        if let Some(p) = check_east(cur, &map) {
            distance_tracker
                .entry(p)
                .and_modify(|x| *x = *x.min(&mut (d + 1)))
                .or_insert(d + 1);
        }
        if let Some(p) = check_west(cur, &map) {
            distance_tracker
                .entry(p)
                .and_modify(|x| *x = *x.min(&mut (d + 1)))
                .or_insert(d + 1);
        }
        if let Some(p) = check_south(cur, &map) {
            distance_tracker
                .entry(p)
                .and_modify(|x| *x = *x.min(&mut (d + 1)))
                .or_insert(d + 1);
        }

        if let Some(x) = distance_tracker
            .iter()
            .filter(|x| !visited.contains(x.0))
            .min_by(|a, b| a.1.cmp(b.1))
        {
            current = Some(*x.0)
        } else {
            current = None
        }
    }

    (
        *distance_tracker
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .1,
        visited,
    )
}

pub fn inside_count_line(line: &Vec<char>, line_num: usize, points: &HashSet<Point>) -> usize {
    let mut count = 0;
    let mut inside = false;

    for (idx, c) in line.iter().enumerate() {
        if points.contains(&(line_num, idx)) {
            match c {
                'L' => inside = !inside,
                'J' => inside = !inside,
                '|' => inside = !inside,
                _ => {}
            }
        } else if inside {
            count += 1;
        }
    }

    count
}

pub fn parse_map(content: &str) -> (Vec<Vec<char>>, Point) {
    let mut res = Vec::new();
    let mut start = (0, 0);
    for (j, line) in content.lines().enumerate() {
        let mut cur = vec![];
        for (i, char) in line.chars().enumerate() {
            cur.push(char);
            if char == 'S' {
                start = (j, i);
            }
        }
        res.push(cur);
    }
    (res, start)
}

#[cfg(test)]
mod tests {
    use crate::download_day;

    use super::{djikstras_furthest, inside_count_line, parse_map};

    const SAMPLE: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn part_one() {
        let content = download_day(2023, 10);
        let (map, start) = parse_map(&content[..]);
        let (res, _) = djikstras_furthest(start, map.clone());
        println!("Part One: {res}");
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 10);
        let (map, start) = parse_map(&content[..]);
        let (_, points) = djikstras_furthest(start, map.clone());
        let res: usize = map
            .iter()
            .enumerate()
            .map(|(idx, line)| inside_count_line(line, idx, &points))
            .sum();
        println!("Part Two: {}", res)
    }

    const SAMPLE_2: &str = "OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO";

    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let (map, start) = parse_map(content);
        let res = djikstras_furthest(start, map);
        assert_eq!(res.0, 8)
    }

    #[test]
    fn part_two_line_test() {
        let content = SAMPLE_2;
        let (map, start) = parse_map(content);
        let (_, points) = djikstras_furthest(start, map.clone());
        let res: usize = map
            .iter()
            .enumerate()
            .map(|(idx, line)| inside_count_line(line, idx, &points))
            .sum();
        println!("{}", res)
    }

    #[test]
    fn map_parse_test() {
        let content = SAMPLE;
        let (map, start) = parse_map(content);
        println!("{map:?}");
        println!("{start:?}");
    }
}
