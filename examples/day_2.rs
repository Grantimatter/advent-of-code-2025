use std::collections::HashMap;

fn main() {
    let (p1, p2) = solution(DAY_2_INPUT);
    println!("Part 1 answer: {p1}");
    println!("Part 2 answer: {p2}");
}

/// Returns invalid IDs in a vector
fn solution(input: &str) -> (u64, u64) {
    let invalid_ids = get_invalid_ids(input);
    let part1_answer = invalid_ids.0.iter().sum();
    let part2_answer = invalid_ids.1.iter().sum();
    // for invalid_id in invalid_ids.1.iter() {
    //     println!("{invalid_id}");
    // }

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
        println!("Start: {start} | End: {end}")
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
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let (pt1, pt2) = solution(DAY_2_EXAMPLE);
        assert_eq!(pt1, 1227775554);
    }

    #[test]
    fn test_example_part_2() {
        let (pt1, pt2) = solution(DAY_2_EXAMPLE);
        assert_eq!(pt2, 4174379265);
    }

    #[test]
    fn test_solution() {
        let (pt1, pt2) = solution(DAY_2_INPUT);
        // Make sure pt1 solution doesn't break
        assert_eq!(pt1, 16793817782);
        assert!(pt2 < 27469417443, "pt2 answer too high");
    }
}

const DAY_2_EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const DAY_2_INPUT: &str = "516015-668918,222165343-222281089,711089-830332,513438518-513569318,4-14,4375067701-4375204460,1490-3407,19488-40334,29275041-29310580,4082818-4162127,12919832-13067769,296-660,6595561-6725149,47-126,5426-11501,136030-293489,170-291,100883-121518,333790-431800,897713-983844,22-41,42727-76056,439729-495565,43918886-44100815,725-1388,9898963615-9899009366,91866251-91958073,36242515-36310763";
