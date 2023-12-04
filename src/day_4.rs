use std::collections::HashSet;

pub fn calculate_matching(card: &str) -> usize {
    let sides: Vec<&str> = card.split("|").collect();
    let winning_side = sides[0];
    let winning_side = winning_side.split(":").collect::<Vec<&str>>()[1];
    let picked_side = sides[1];
    let mut winning_iter = winning_side.split_whitespace();

    let mut winning: HashSet<usize> = HashSet::new();
    while let Some(num) = winning_iter.next() {
        winning.insert(num.parse().expect("expected number"));
    }

    let mut matching = 0;
    let mut picked_iter = picked_side.split_whitespace();
    while let Some(num) = picked_iter.next() {
        let num = num.parse().expect("expected number");
        if winning.contains(&num) {
            matching += 1;
        }
    }

    matching
}

pub fn calculate_copies(cards: &str) -> usize {
    let score: Vec<usize> = cards.lines().map(calculate_matching).collect();
    let mut copies = vec![1; score.len()];
    for i in 0..copies.len() {
        let amount = copies[i];
        for j in i + 1..=i + score[i] as usize {
            copies[j] += amount
        }
    }

    copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::download_day;

    use super::{calculate_copies, calculate_matching};

    #[test]
    fn part_one() {
        let content = download_day(2023, 4);
        let res: u32 = content
            .lines()
            .map(calculate_matching)
            .map(|amount| if amount == 0 { 0 } else { 1 << (amount - 1) })
            .sum();
        println!("Part 1: {res}");
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 4);
        let res = calculate_copies(&content);
        println!("Part two: {res}");
    }

    #[test]
    fn score_test() {
        let card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let res = calculate_matching(card);
        assert_eq!(
            {
                if res == 0 {
                    0
                } else {
                    1 << (res - 1)
                }
            },
            8
        );
    }

    #[test]
    fn copies_test() {
        let content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = calculate_copies(content);
        assert_eq!(res, 30);
    }

    #[test]
    fn calculate_pile() {
        let content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res: u32 = content
            .lines()
            .map(calculate_matching)
            .map(|amount| if amount == 0 { 0 } else { 1 << (amount - 1) })
            .sum();
        assert_eq!(res, 13);
    }
}
