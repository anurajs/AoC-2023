use std::collections::HashMap;

#[derive(Debug, Clone, Hash)]
pub struct Lens {
    label: String,
    value: usize,
}

pub fn holiday_ascii_string_helper_manual_arrangement_procedure(
    content: &str,
    map: &mut HashMap<usize, Vec<Lens>>,
) {
    if content.contains("=") {
        let mut splitter = content.split("=");
        let label = splitter.nth(0).unwrap().to_string();
        let value = splitter.nth(0).unwrap().parse::<usize>().unwrap();
        let box_loc = holiday_ascii_string_helper(&label);
        let lens = Lens { label, value };
        map.entry(box_loc)
            .and_modify(|vec| {
                if let Some(idx) = vec.iter().position(|x| x.label == lens.label) {
                    vec[idx].value = lens.value;
                    return;
                }
                vec.push(lens.clone());
                return;
            })
            .or_insert(vec![lens.clone()]);
    } else {
        let label = content.split("-").nth(0).unwrap().to_string();
        let box_loc = holiday_ascii_string_helper(&label);
        map.entry(box_loc)
            .and_modify(|vec| {
                if let Some(idx) = vec.iter().position(|x| x.label == label) {
                    vec.remove(idx);
                }
            })
            .or_insert(Vec::new());
    }
}

pub fn holiday_ascii_string_helper(content: &str) -> usize {
    let mut total: usize = 0;
    for c in content.chars() {
        total += (c as u8) as usize;
        total *= 17;
        total %= 256;
    }

    total
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{day_15::holiday_ascii_string_helper, download_day};

    use super::holiday_ascii_string_helper_manual_arrangement_procedure;

    #[test]
    fn part_one() {
        let res = download_day(2023, 15)
            .split(",")
            .map(|x| holiday_ascii_string_helper(x.trim()))
            .sum::<usize>();

        println!("Part One: {res}")
    }

    #[test]
    fn part_two() {
        let content = download_day(2023, 15);
        let mut map = HashMap::new();
        content.split(",").for_each(|x| {
            holiday_ascii_string_helper_manual_arrangement_procedure(x.trim(), &mut map);
        });
        let mut total = 0;
        for (box_num, lens_list) in map {
            for (idx, lens) in lens_list.iter().enumerate() {
                total += (idx + 1) * lens.value * (box_num + 1);
            }
        }
        println!("Part Two: {total}");
    }

    const SAMPLE: &str = "HASH";
    const SAMPLE_2: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn part_one_sample() {
        assert_eq!(
            1320,
            SAMPLE_2
                .split(",")
                .map(holiday_ascii_string_helper)
                .sum::<usize>()
        )
    }
    #[test]
    fn part_two_sample() {
        let mut map = HashMap::new();
        let content = SAMPLE_2;
        content.split(",").for_each(|x| {
            holiday_ascii_string_helper_manual_arrangement_procedure(x.trim(), &mut map);
        });
        let mut total = 0;
        for (box_num, lens_list) in map {
            for (idx, lens) in lens_list.iter().enumerate() {
                total += (idx + 1) * lens.value * (box_num + 1);
            }
        }
        assert_eq!(total, 145);
    }

    #[test]
    fn hash_sample() {
        assert_eq!(holiday_ascii_string_helper(SAMPLE), 52);
    }
}
