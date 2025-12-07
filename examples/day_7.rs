fn main() {
    let input = std::fs::read_to_string("./inputs/day_7").unwrap();
    let (answer_pt1, _) = solution(&input);
    println!("Answer Part 1 {answer_pt1}");
}

fn solution(input: &str) -> (u64, u64) {
    todo!();
}

#[cfg(test)]
mod day_7_tests {
    use super::*;

    #[test]
    fn test_example_pt1() {
        let input = std::fs::read_to_string("./inputs/day_7_example").unwrap();
        let (answer_pt1, _) = solution(&input);
        assert_eq!(21, answer_pt1)
    }
}
