pub fn extrapolate_last_number(numbers: &[isize]) -> isize {
    let first = numbers[0];

    if numbers.iter().copied().all(|x| x == first) {
        return first;
    }

    let next: Vec<_> = numbers
        .iter()
        .as_slice()
        .windows(2)
        .map(|nums| nums[1] - nums[0])
        .collect();

    return numbers[numbers.len() - 1] + extrapolate_last_number(&next);
}

pub fn extrapolate_first_number(numbers: &[isize]) -> isize {
    let first = numbers[0];

    if numbers.iter().copied().all(|x| x == first) {
        return first;
    }

    let next: Vec<_> = numbers
        .iter()
        .as_slice()
        .windows(2)
        .map(|nums| nums[1] - nums[0])
        .collect();

    return numbers[0] - extrapolate_first_number(&next);
}

#[cfg(test)]
mod tests {
    use crate::{
        day_9::{extrapolate_first_number, extrapolate_last_number},
        download_day,
    };

    const SAMPLE: &str = "10  13  16  21  30  45";

    #[test]
    fn part_two() {
        let content = download_day(2023, 9);
        let res: isize = content
            .lines()
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .map(|nums| extrapolate_first_number(&nums))
            .sum();

        println!("Part One: {res}");
    }

    #[test]
    fn part_one() {
        let content = download_day(2023, 9);
        let res: isize = content
            .lines()
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .map(|nums| extrapolate_last_number(&nums))
            .sum();

        println!("Part One: {res}");
    }

    #[test]
    fn part_one_sample() {
        let content = SAMPLE;
        let numbers: Vec<_> = content
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(extrapolate_last_number(&numbers), 68);
    }

    #[test]
    fn part_two_sample() {
        let content = SAMPLE;
        let numbers: Vec<_> = content
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(extrapolate_first_number(&numbers), 5);
    }
}
