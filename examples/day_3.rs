fn main() {
    let input = std::fs::read_to_string("./inputs/day_3").expect("Should be able to read the file");
    let result_pt1 = solution_pt1(&input);
    let result_pt2 = solution_pt2(&input);

    println!("pt1 answer: {result_pt1}");
    println!("pt2 answer: {result_pt2}");
}

fn solution_pt1(input: &str) -> u64 {
    let bank_list: Vec<&str> = input.lines().collect();
    let mut jolts: Vec<u64> = Vec::new();
    for bank in bank_list {
        // Each digit in the bank is a battery
        let mut highest_start_joltage: u64 = 0;
        let mut highest_joltage: u64 = 0;
        for (idx, battery) in bank.chars().enumerate() {
            if idx < bank.len() - 1 && battery as u64 > highest_start_joltage {
                highest_start_joltage = battery.to_digit(10).unwrap() as u64;
                for r_bat in bank[idx + 1..].chars() {
                    let mut current_joltage = highest_start_joltage.to_string();
                    current_joltage.push(r_bat);
                    let current_joltage = current_joltage.parse::<u64>().unwrap();
                    if current_joltage > highest_joltage {
                        highest_joltage = current_joltage;
                    }
                }
            }
        }
        jolts.push(highest_joltage);
    }

    jolts.iter().sum()
}

fn solution_pt2(input: &str) -> u64 {
    let bank_list: Vec<&str> = input.lines().collect();
    let mut jolts: Vec<u64> = Vec::new();
    for bank in bank_list {
        // Each digit in the bank is a battery
        let mut remaining_cells: usize = 12;
        let mut joltage: String = String::from("");
        let mut idx = 0;
        while remaining_cells > 0 {
            let (j, i) = next_highest_possible(&bank[idx..], remaining_cells);
            joltage += &(j.to_string());
            idx += i;
            remaining_cells -= 1;
        }
        jolts.push(
            joltage
                .parse::<u64>()
                .expect("Should be able to parse an integer"),
        );
    }
    jolts.iter().sum()
}

fn next_highest_possible(bank: &str, remaining: usize) -> (u64, usize) {
    let mut highest_reamining = 0;
    let mut highest_index = 0;
    for (idx, battery) in bank.chars().enumerate() {
        let cell_value = battery.to_digit(10).unwrap() as u64;
        if idx < bank.len() - remaining + 1 && cell_value > highest_reamining {
            highest_reamining = cell_value;
            highest_index = idx + 1;
        }
    }
    (highest_reamining, highest_index)
}

#[cfg(test)]
#[allow(warnings)]
mod test {
    use super::*;

    #[test]
    #[allow(warnings)]
    fn test_example_pt1() {
        let input = std::fs::read_to_string("./inputs/day_3_example").unwrap();
        let result = solution_pt1(&input);
        assert_eq!(result, 357);
    }

    #[test]
    #[allow(warnings)]
    fn test_example_pt2() {
        let input = std::fs::read_to_string("./inputs/day_3_example").unwrap();
        let result = solution_pt2(&input);
        assert_eq!(result, 3121910778619);
    }
}
