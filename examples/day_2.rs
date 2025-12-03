use std::collections::HashMap;

fn main() {
    let input =
        std::fs::read_to_string("./inputs/day_2").expect("Should have been able to read the file");
    let (pt1, pt2) = solution(input.trim());
    println!("Part 1 answer: {pt1}");
    println!("Part 2 answer: {pt2}");
}

fn solution(input: &str) -> (u64, u64) {
    let invalid_ids = get_invalid_ids(input);
    let part1_answer = invalid_ids.0.iter().sum();
    let part2_answer = invalid_ids.1.iter().sum();

    (part1_answer, part2_answer)
}

fn get_invalid_ids(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut invalid_ids_pt1: Vec<u64> = Vec::new();
    let mut invalid_ids_pt2: Vec<u64> = Vec::new();
    let ranges = input.split(",");
    for range in ranges {
        let r: Vec<&str> = range.split("-").collect();
        let start = r[0].parse::<u64>().unwrap();
        let end = r[1].parse::<u64>().unwrap();
        for i in start..=end {
            if is_repeating_pt1(i) {
                invalid_ids_pt1.push(i);
            }
            if is_repeating_pt2(i) {
                invalid_ids_pt2.push(i);
            }
        }
    }
    (invalid_ids_pt1, invalid_ids_pt2)
}

fn is_repeating_pt1(id: u64) -> bool {
    let id_str = id.to_string();
    let sections = id_str.split_at(id_str.len() / 2);
    sections.0 == sections.1
}

fn is_repeating_pt2(id: u64) -> bool {
    let id_str = id.to_string();
    let mut chunk_size = 1;
    loop {
        let sections: Vec<String> = id_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect();
        let mut section_count: HashMap<&str, u32> = HashMap::new();
        for section in sections.iter() {
            *section_count.entry(section).or_insert(0) += 1
        }
        if section_count.len() == 1 && sections.len() > 1 {
            return true;
        }
        chunk_size += 1;
        if chunk_size > id_str.len() / 2 {
            break;
        }
    }

    false
}

#[cfg(test)]
#[allow(warnings)]
mod tests {
    use super::*;

    // #[test]
    #[allow(warnings)]
    fn test_example_part_1() {
        let input = std::fs::read_to_string("./inputs/day_2_example").unwrap();
        let (pt1, _) = solution(input.trim());
        assert_eq!(pt1, 1227775554);
    }

    // #[test]
    #[allow(warnings)]
    fn test_example_part_2() {
        let input = std::fs::read_to_string("./inputs/day_2_example").unwrap();
        let (_, pt2) = solution(input.trim());
        assert_eq!(pt2, 4174379265);
    }

    // #[test]
    #[allow(warnings)]
    fn test_solution() {
        let input = std::fs::read_to_string("./inputs/day_2").unwrap();
        let (pt1, pt2) = solution(input.trim());
        // Make sure pt1 solution doesn't break
        assert_eq!(pt1, 16793817782);
        assert!(pt2 < 27469417443, "pt2 answer too high");
    }
}
