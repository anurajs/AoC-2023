use std::{collections::HashMap, usize};

pub fn get_arrangements(
    line: String,
    sequence: &[usize],
    pos: usize,
    group: usize,
    group_len: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if cache.contains_key(&(pos, group, group_len)) {
        return cache[&(pos, group, group_len)];
    }
    if pos == line.len() {
        if group == sequence.len() && group_len == 0
            || (group == sequence.len() - 1 && group_len == sequence[group])
        {
            cache.insert((pos, group, group_len), 1);
            return 1;
        } else {
            cache.insert((pos, group, group_len), 0);
            return 0;
        };
    }
    let mut arrangements = 0;

    let current: u8 = line.bytes().collect::<Vec<u8>>()[pos];
    if b".?".contains(&current) {
        if group < sequence.len() && group_len == sequence[group] {
            arrangements += get_arrangements(line.clone(), sequence, pos + 1, group + 1, 0, cache)
        }
        if group_len == 0 {
            arrangements +=
                get_arrangements(line.clone(), sequence, pos + 1, group, group_len, cache)
        }
    }

    if b"#?".contains(&current) {
        arrangements +=
            get_arrangements(line.clone(), sequence, pos + 1, group, group_len + 1, cache)
    }

    cache.insert((pos, group, group_len), arrangements);
    return arrangements;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::download_day;

    use super::get_arrangements;

    #[test]
    fn part_one() {
        let content = download_day(2023, 12);
        let res: usize = content
            .lines()
            .map(|s| s.split_whitespace())
            .map(|mut split| {
                let line = split.next().unwrap().to_string();
                let sequence: Vec<usize> = split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect();
                (line, sequence)
            })
            .map(|(line, sequence)| get_arrangements(line, &sequence, 0, 0, 0, &mut HashMap::new()))
            .sum();
        println!("Part One: {res}");
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 12);
        let res: usize = content
            .lines()
            .map(|s| (s.split_whitespace()))
            .map(|mut split| {
                let mut line: String = split.next().unwrap().to_string();
                line = (0..5).map(|_| &line[..]).collect::<Vec<&str>>().join("?");
                let sequence: Vec<usize> = split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect();
                let sequence = sequence.repeat(5);
                (line, sequence)
            })
            .map(|(line, sequence)| {
                let mut cache = HashMap::new();
                let res = get_arrangements(line, &sequence, 0, 0, 0, &mut cache);
                res
            })
            .sum();
        println!("Part Two: {res}");
    }

    const SAMPLE: &str = "?###???????? 3,2,1
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5";
    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let res: usize = content
            .lines()
            .enumerate()
            .map(|s| (s.0, s.1.split_whitespace()))
            .map(|(count, mut split)| {
                let line = split.next().unwrap().to_string();
                println!("{count} {line}");
                let sequence: Vec<usize> = split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect();
                (line, sequence)
            })
            .map(|(line, sequence)| get_arrangements(line, &sequence, 0, 0, 0, &mut HashMap::new()))
            .sum();
        assert_eq!(res, 21)
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE;
        let res: usize = content
            .lines()
            .map(|s| s.split_whitespace())
            .map(|mut split| {
                let mut line: String = split.next().unwrap().to_string();
                line.push_str("?");
                line = line.repeat(5);
                let sequence: Vec<usize> = split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect();
                let sequence = sequence.repeat(5);
                (line[0..line.len() - 1].to_string(), sequence)
            })
            .map(|(line, sequence)| get_arrangements(line, &sequence, 0, 0, 0, &mut HashMap::new()))
            .sum();
        assert_eq!(res, 525152)
    }

    #[test]
    fn get_arrangements_test() {
        let line = "???.###".to_string();
        let sequence = [1, 1, 3];
        let mut cache = HashMap::new();
        let res = get_arrangements(line, &sequence, 0, 0, 0, &mut cache);
        assert_eq!(res, 1);
    }

    #[test]
    fn replacement_test() {
        let mut line: String = "???.###".to_string();
        line.push_str("?");
        line = line.repeat(5);
        let line = &line[0..&line.len() - 1];
        assert_eq!(line, "???.###????.###????.###????.###????.###");
        let sequence = [1, 1, 3];
        let sequence = sequence.repeat(5);
        assert_eq!(sequence, [1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]);
    }
}
