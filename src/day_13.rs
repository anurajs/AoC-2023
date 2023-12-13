pub fn find_reflection(map: &[Vec<char>], differences: usize) -> usize {
    for i in 1..map.len() {
        let mut diff = 0;
        'outer: for (a, b) in (i..map.len()).zip((0..i).rev()) {
            for j in 0..map[0].len() {
                if map[a][j] != map[b][j] {
                    diff += 1;
                    if diff > differences {
                        break 'outer;
                    }
                }
            }
        }
        if diff == differences {
            return i * 100;
        }
    }

    for i in 1..map[0].len() {
        let mut diff = 0;
        'outer: for (a, b) in (i..map[0].len()).zip((0..i).rev()) {
            for j in 0..map.len() {
                if map[j][a] != map[j][b] {
                    diff += 1;
                    if diff > differences {
                        break 'outer;
                    }
                }
            }
        }
        if diff == differences {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::{day_13::find_reflection, download_day};

    const SAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part_one() {
        let content = download_day(2023, 13);
        let mut patterns = vec![];
        let mut pattern = vec![];
        for line in content.lines() {
            if line.is_empty() {
                patterns.push(pattern);
                pattern = vec![];
            } else {
                pattern.push(line.chars().collect::<Vec<char>>());
            }
        }
        patterns.push(pattern);
        let res = patterns
            .iter()
            .map(|x| find_reflection(x, 0))
            .sum::<usize>();
        println!("Part One: {res}");
    }
    #[test]
    fn part_two() {
        let content = download_day(2023, 13);
        let mut patterns = vec![];
        let mut pattern = vec![];
        for line in content.lines() {
            if line.is_empty() {
                patterns.push(pattern);
                pattern = vec![];
            } else {
                pattern.push(line.chars().collect::<Vec<char>>());
            }
        }
        patterns.push(pattern);
        let res = patterns
            .iter()
            .map(|x| find_reflection(x, 1))
            .sum::<usize>();
        println!("Part Two: {res}");
    }
    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let mut patterns = vec![];
        let mut pattern = vec![];
        for line in content.lines() {
            if line.is_empty() {
                patterns.push(pattern);
                pattern = vec![];
            } else {
                pattern.push(line.chars().collect::<Vec<char>>());
            }
        }
        patterns.push(pattern);
        assert_eq!(
            patterns
                .iter()
                .map(|x| find_reflection(x, 0))
                .sum::<usize>(),
            405
        );
        assert_eq!(find_reflection(&patterns[0], 0), 5);
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE;
        let mut patterns = vec![];
        let mut pattern = vec![];
        for line in content.lines() {
            if line.is_empty() {
                patterns.push(pattern);
                pattern = vec![];
            } else {
                pattern.push(line.chars().collect::<Vec<char>>());
            }
        }
        patterns.push(pattern);
        assert_eq!(
            patterns
                .iter()
                .map(|x| find_reflection(x, 1))
                .sum::<usize>(),
            400
        );
        assert_eq!(find_reflection(&patterns[0], 1), 300);
    }
}
