pub fn new_records(time: usize, current_record: usize) -> Vec<usize> {
    let mut better_times = vec![];
    for i in 0..=time {
        let speed = i;
        let distance = (time - i) * speed;
        if distance > current_record {
            better_times.push(i);
        } else if better_times.len() > 0 {
            break;
        }
    }
    println!("{}", better_times.len());
    better_times
}

pub fn new_records_math(time: usize, current_record: usize) -> usize {
    let a: i64 = -1;
    let b: i64 = time as i64;
    let c = -(current_record as i64);
    let discriminant = ((b.pow(2) - 4 * a * c) as f64).sqrt();
    if discriminant <= 0f64 {
        return 0;
    }
    let root_one = (-b as f64 + discriminant) / 2f64 * a as f64;
    let root_two = (-b as f64 - discriminant) / 2f64 * a as f64;
    (root_two.ceil() - (root_one + 1f64).floor()) as usize
}

#[cfg(test)]
mod tests {
    use crate::{day_6::new_records_math, download_day, timeit};

    #[test]
    fn part_one() {
        let content = download_day(2023, 6);
        let times: Vec<usize> = content
            .lines()
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let distances: Vec<usize> = content
            .lines()
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let res = times
            .iter()
            .copied()
            .zip(distances)
            .map(|(time, distance)| new_records_math(time, distance))
            .fold(1, |acc, x| acc * x);
        println!("Part 1: {res}")
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 6);
        let time: usize = content
            .lines()
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .fold(String::new(), |mut s, n| {
                s.push_str(n);
                s
            })
            .parse()
            .unwrap();
        let distance: usize = content
            .lines()
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .fold(String::new(), |mut s, n| {
                s.push_str(n);
                s
            })
            .parse()
            .unwrap();
        let res = new_records_math(time, distance);
        println!("Part 2: {}", res);
    }

    #[test]
    fn part_one_sample() {
        let content = "Time:      7  15   30
Distance:  9  40  200";
        let times: Vec<usize> = content
            .lines()
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let distances: Vec<usize> = content
            .lines()
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let res = times
            .iter()
            .zip(distances)
            .map(|(time, distance)| new_records_math(*time, distance))
            .fold(1, |acc, x| acc * x);
        assert_eq!(res, 288)
    }

    #[test]
    fn part_two_sample() {
        let content = "Time:      7  15   30
Distance:  9  40  200";
        let time: usize = content
            .lines()
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .fold(String::new(), |mut s, n| {
                s.push_str(n);
                s
            })
            .parse()
            .unwrap();
        let distance: usize = content
            .lines()
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .fold(String::new(), |mut s, n| {
                s.push_str(n);
                s
            })
            .parse()
            .unwrap();
        let res = new_records_math(time, distance);
        assert_eq!(res, 71503)
    }

    #[test]
    fn test_math() {
        println!("{}", new_records_math(7, 9));
    }

    #[test]
    fn time_part_two() {
        timeit(part_two);
    }
}
