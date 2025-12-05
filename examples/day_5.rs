use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let input = std::fs::read_to_string("./inputs/day_5").expect("Failed to read the file");
    let (answer_pt1, answer_pt2) = solution(&input);
    println!("Part 1 Answer: {answer_pt1}");
    println!("Part 2 Answer: {answer_pt2}");
}

#[allow(warnings)]
fn solution(input: &str) -> (u64, u64) {
    // Return the sum of the fresh ingredient counter
    let (mut fresh_id_ranges, ingredient_ids) = input
        .split_once("\n\n")
        .map(|n| {
            (
                n.0.lines()
                    .map(|l| {
                        l.split_once("-")
                            .map(|r| {
                                let (start, end) =
                                    (r.0.parse::<u64>().unwrap(), r.1.parse::<u64>().unwrap());

                                start..=end
                            })
                            .unwrap()
                    })
                    .collect::<Vec<RangeInclusive<u64>>>(),
                n.1.lines()
                    .map(|l| l.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .unwrap();

    for i in 0..fresh_id_ranges.len() {
        if i == fresh_id_ranges.len() - 1 {
            break;
        }

        let start_in: Option<&RangeInclusive<u64>> = fresh_id_ranges
            .iter()
            .filter(|r| **r != fresh_id_ranges[i] && r.contains(fresh_id_ranges[i].start()))
            .reduce(|acc, o| if acc.end() > o.end() { acc } else { o });

        let end_in: Option<&RangeInclusive<u64>> = fresh_id_ranges
            .iter()
            .filter(|r| **r != fresh_id_ranges[i] && r.contains(fresh_id_ranges[i].end()))
            .reduce(|acc, o| if acc.start() < o.start() { acc } else { o });

        let start = if start_in.is_some_and(|s| !s.is_empty()) {
            start_in.unwrap().end() + 1
        } else {
            *fresh_id_ranges[i].start()
        };

        let end = if end_in.is_some_and(|e| !e.is_empty()) {
            end_in.unwrap().start() - 1
        } else {
            *fresh_id_ranges[i].end()
        };

        fresh_id_ranges[i] = start..=end;
    }

    let fresh_id_set: HashSet<&RangeInclusive<u64>> = fresh_id_ranges.iter().collect();

    let mut count: u64 = 0;
    fresh_id_set
        .iter()
        .for_each(|f| count += f.clone().clone().count() as u64);

    let mut fresh_ingredient_count = 0;
    ingredient_ids.iter().for_each(|id| {
        if is_ingredient_fresh(id, &fresh_id_ranges) {
            fresh_ingredient_count += 1;
        }
    });

    (fresh_ingredient_count, count)
}

fn is_ingredient_fresh(ingredient_id: &u64, ranges: &[RangeInclusive<u64>]) -> bool {
    ranges.iter().any(|r| r.contains(ingredient_id))
}

#[cfg(test)]
mod day_5_tests {
    use super::*;

    #[test]
    fn test_example_pt1() {
        let input = std::fs::read_to_string("./inputs/day_5_example").unwrap();
        let (answer_pt1, _) = solution(&input);
        assert_eq!(answer_pt1, 3);
    }

    #[test]
    fn test_example_pt2() {
        let input = std::fs::read_to_string("./inputs/day_5_example").unwrap();
        let (_, answer_pt2) = solution(&input);
        assert_eq!(answer_pt2, 14);
    }
}
