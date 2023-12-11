pub fn part_x(map: &Vec<Vec<char>>, expansion_factor: isize) -> usize {
    let mut empty_rows = vec![true; map.len()];
    let mut empty_columns = vec![true; map[0].len()];
    let mut points = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                points.push((i as isize, j as isize));
                empty_columns[j] = false;
                empty_rows[i] = false;
            }
        }
    }

    let mut distances = vec![];
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let mut distance =
                (points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs();

            distance += empty_rows
                .iter()
                .skip(points[i].0 as usize)
                .take((points[i].0 - points[j].0).abs() as usize)
                .filter(|x| **x)
                .count() as isize
                * 1.max(expansion_factor - 1);

            distance += empty_columns
                .iter()
                .skip(points[i].1.min(points[j].1) as usize)
                .take((points[i].1 - points[j].1).abs() as usize)
                .filter(|x| **x)
                .count() as isize
                * 1.max(expansion_factor - 1);
            distances.push(distance);
        }
    }
    distances.iter().sum::<isize>() as usize
}
pub fn parse_map(content: &str) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    for line in content.lines() {
        let mut cur = vec![];
        for char in line.chars() {
            cur.push(char);
        }
        res.push(cur);
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::download_day;

    use super::{parse_map, part_x};

    static SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part_one_test() {
        let content = download_day(2023, 11);
        let map = parse_map(&content[..]);
        let res = part_x(&map, 1);
        println!("Part One: {res}");
    }
    #[test]
    fn part_two_test() {
        let content = download_day(2023, 11);
        let map = parse_map(&content[..]);
        let res = part_x(&map, 1000000);
        println!("Part One: {res}");
    }

    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let map = parse_map(content);
        let res = part_x(&map, 1);
        assert_eq!(res, 374);
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE;
        let map = parse_map(content);
        let res = part_x(&map, 100);
        assert_eq!(res, 8410);
    }
}
