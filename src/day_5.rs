use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Map {
    source: usize,
    destination: usize,
    range: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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

pub fn ranges_in_map(
    location_range: LocationRange,
    map: &Map,
) -> (HashSet<LocationRange>, HashSet<LocationRange>) {
    let mut covered = HashSet::new();
    let mut not_covered = HashSet::new();
    if location_range.location < map.source {
        not_covered.insert(LocationRange {
            location: location_range.location,
            range: (location_range.range).min(map.source - location_range.location),
        });
        if map.source - location_range.location < location_range.range {
            covered.insert(LocationRange {
                location: map.source,
                range: map
                    .range
                    .min(location_range.location + location_range.range - map.source),
            });
            if map.source + map.range < location_range.location + location_range.range {
                not_covered.insert(LocationRange {
                    location: map.source + map.range,
                    range: location_range.location + location_range.range - map.source - map.range,
                });
            }
        };
    }

    if location_range.location >= map.source && location_range.location < map.source + map.range {
        covered.insert(LocationRange {
            location: location_range.location,
            range: location_range
                .range
                .min(map.source + map.range - location_range.location),
        });
        if location_range.range > map.source + map.range - location_range.location {
            not_covered.insert(LocationRange {
                location: map.source + map.range,
                range: location_range.range + location_range.location - map.source - map.range,
            });
        }
    } else if location_range.location >= map.source + map.range {
        not_covered.insert(location_range);
    }

    (covered, not_covered)
}

fn is_range_covered(range: LocationRange, map: &Map) -> Option<LocationRange> {
    if range.location >= map.source && range.location + range.range <= map.source + map.range {
        return Some(LocationRange {
            location: map.destination + (range.location - map.source),
            range: range.range,
        });
    }

    None
}

pub fn calculate_location_ranges(
    initial: HashSet<LocationRange>,
    maps: &[&str],
) -> HashSet<LocationRange> {
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

    let mut covered: HashSet<LocationRange> = HashSet::new();
    let mut not_covered: HashSet<LocationRange> = HashSet::from(initial);
    loop {
        let num_uncovered = not_covered.len();
        let mut temp_not_covered = HashSet::new();
        'unc: for uncovered in not_covered.drain() {
            for map in map_collection.iter() {
                let (mut c, mut u) = ranges_in_map(uncovered, map);
                let c_len = c.len();
                covered.extend(c.drain());
                temp_not_covered.extend(u.drain());
                if c_len > 0 {
                    temp_not_covered.remove(&uncovered);
                    continue 'unc;
                }
            }
        }
        not_covered = temp_not_covered.difference(&covered).copied().collect();
        if not_covered.len() == num_uncovered {
            break;
        }
    }

    let mut covered_to_min: HashMap<LocationRange, LocationRange> = HashMap::new();
    for c in covered {
        for map in map_collection.iter() {
            match is_range_covered(c, map) {
                Some(location) => {
                    covered_to_min
                        .entry(c)
                        .and_modify(|c| {
                            if location.location < c.location {
                                c.location = location.location;
                                c.range = location.range;
                            }
                        })
                        .or_insert(location);
                }
                None => {}
            }
        }
    }

    // not_covered.extend(covered_to_min.values())
    let min_covered: HashSet<LocationRange> = covered_to_min.into_values().collect();
    not_covered.extend(min_covered);
    not_covered
}

pub fn calculate_location(initial: HashSet<usize>, maps: &[&str]) -> HashSet<usize> {
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

    use crate::{
        day_5::{calculate_location, calculate_location_ranges, LocationRange},
        download_day, timeit,
    };

    #[test]
    fn part_two() {
        let content = download_day(2023, 5);
        let mut line_iter = content.lines();
        let ranges: Vec<usize> = line_iter
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let mut initial = HashSet::new();
        for x in ranges.windows(2).step_by(2) {
            initial.insert(LocationRange {
                location: x[0],
                range: x[1],
            });
        }

        let mut maps: Vec<&str> = Vec::new();
        line_iter.next();
        line_iter.next();
        while let Some(line) = line_iter.next() {
            if !line.trim().is_empty() {
                maps.push(line);
            } else {
                initial = calculate_location_ranges(initial, &maps);
                line_iter.next();
                maps.clear();
            }
        }

        let res = calculate_location_ranges(initial, &maps);

        println!(
            "Part 2: {}",
            res.iter().fold(usize::MAX, |acc, x| acc.min(x.location))
        )
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
        let maps: Vec<&str> = "50 98 2
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

    #[test]
    fn part_two_sample() {
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
        let ranges: Vec<usize> = line_iter
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let mut initial = HashSet::new();
        for x in ranges.windows(2).step_by(2) {
            initial.insert(LocationRange {
                location: x[0],
                range: x[1],
            });
        }

        let mut maps: Vec<&str> = Vec::new();
        line_iter.next();
        line_iter.next();
        while let Some(line) = line_iter.next() {
            if !line.trim().is_empty() {
                maps.push(line);
            } else {
                initial = calculate_location_ranges(initial, &maps);
                line_iter.next();
                maps.clear();
            }
        }

        let res = calculate_location_ranges(initial, &maps);

        println!(
            "Part 2: {}",
            res.iter().fold(usize::MAX, |acc, x| acc.min(x.location))
        )
    }

    #[test]
    fn time_part_two() {
        timeit(part_two)
    }
}
