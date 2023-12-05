use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Map {
    source: usize,
    destination: usize,
    range: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct LocationRange {
    location: usize,
    range: usize,
}

pub fn is_in_range(map: &Map, initial: usize) -> Option<usize> {
    if initial >= map.source && initial < map.source + map.range {
        let location = map.destination + (initial - map.source);
        return Some(location);
    }
    None
}

pub fn calculate_location(initial: HashSet<usize>, maps: &Vec<&str>) -> HashSet<usize> {
    let mut map_collection: Vec<Map> = Vec::new();
    for line in maps {
        let mut parts = line.split_whitespace();
        let destination = parts.next().unwrap().parse().unwrap();
        let source = parts.next().unwrap().parse().unwrap();
        let range = parts.next().unwrap().parse().unwrap();
        map_collection.push(Map {
            source,
            range,
            destination,
        });
    }

    let mut source_to_min: HashMap<usize, usize> = HashMap::new();
    for source in initial.iter() {
        for map in map_collection.iter() {
            match is_in_range(&map, *source) {
                Some(mut location) => {
                    source_to_min
                        .entry(*source)
                        .and_modify(|e| *e = *e.min(&mut location))
                        .or_insert(location);
                }
                None => {}
            }
        }
    }

    for source in initial {
        source_to_min.entry(source).or_insert(source);
    }

    source_to_min.values().map(|v| *v).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{day_5::calculate_location, download_day};

    #[test]
    fn part_two() {
        let content = download_day(2023, 5);
        let mut line_iter = content.lines();
        let mut ranges: Vec<usize> = line_iter
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let mut initial = HashSet::new();
        for x in ranges.windows(2) {
            for i in x[0]..x[0] + x[1] {
                initial.insert(i);
            }
        }

        let mut maps: Vec<&str> = Vec::new();
        line_iter.next();
        line_iter.next();
        while let Some(line) = line_iter.next() {
            if !line.trim().is_empty() {
                maps.push(line);
            } else {
                initial = calculate_location(initial, &maps);
                maps.clear();
                line_iter.next();
            }
        }

        let res = calculate_location(initial, &maps);
        println!("Part 2: {}", res.iter().min().unwrap())
    }

    #[test]
    fn part_one() {
        let content = download_day(2023, 5);
        let mut line_iter = content.lines();
        let mut initial: HashSet<usize> = line_iter
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let mut maps: Vec<&str> = Vec::new();

        line_iter.next();
        line_iter.next();
        while let Some(line) = line_iter.next() {
            if !line.trim().is_empty() {
                maps.push(line);
            } else {
                initial = calculate_location(initial, &maps);
                maps.clear();
                line_iter.next();
            }
        }

        let res = calculate_location(initial, &maps);
        println!("Part 1: {}", res.iter().min().unwrap())
    }

    #[test]
    fn calculate_location_test() {
        let initial: HashSet<usize> = HashSet::from([79, 14, 55, 13]);
        let maps = "50 98 2
52 50 48"
            .lines()
            .collect();
        let res = calculate_location(initial, &maps);
        assert_eq!(res, HashSet::from([81, 14, 57, 13]));
    }

    #[test]
    fn part_one_sample() {
        let content = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let mut line_iter = content.lines();
        let mut initial: HashSet<usize> = line_iter
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let mut maps: Vec<&str> = Vec::new();

        line_iter.next();
        line_iter.next();
        while let Some(line) = line_iter.next() {
            if !line.trim().is_empty() {
                maps.push(line);
            } else {
                initial = calculate_location(initial, &maps);
                maps.clear();
                line_iter.next();
            }
        }

        let res = calculate_location(initial, &maps);
        assert_eq!(res, HashSet::from([82, 43, 86, 35]));
    }
}
