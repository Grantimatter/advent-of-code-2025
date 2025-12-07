fn main() {
    let input = std::fs::read_to_string("./inputs/day_6").expect("Failed to read input file");
    let (answer_pt1, answer_pt2) = solution(&input);
    println!("Answer Part 1: {answer_pt1}");
    println!("Answer Part 2: {answer_pt2}");
}

fn solution(input: &str) -> (u64, u64) {
    let table = parse_table(input);
    let solution_pt1 = table
        .iter()
        .map(|c| c.solution_pt1())
        .reduce(|acc, c| acc + c)
        .unwrap();

    let solution_pt2 = table
        .iter()
        .map(|c| c.solution_pt2())
        .reduce(|acc, c| acc + c)
        .unwrap();

    (solution_pt1, solution_pt2)
}

enum Operator {
    Add,
    Multiply,
}

struct Column {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Column {
    pub fn solution_pt1(&self) -> u64 {
        match self.operator {
            Operator::Add => self.numbers.iter().sum(),
            Operator::Multiply => self.numbers.iter().product(),
        }
    }

    pub fn solution_pt2(&self) -> u64 {
        println!("{:?}", &self.numbers.iter().rev().collect::<Vec<&u64>>());
        // for val in &self.numbers {
        //     println!("{:?}", val);
        // }
        println!("\n");
        0
        // let solution = numbers.
        // match self.operator {
        //     Operator::Add => self.numbers.iter().sum(),
        //     Operator::Multiply => self.numbers.iter().product(),
        // }
    }

    pub fn new(values: &[u64], operator: &str) -> Column {
        Column {
            numbers: Vec::from(values),
            operator: match operator {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                _ => panic!("Unkown operator"),
            },
        }
    }
}

fn parse_table(input: &str) -> Vec<Column> {
    let cells = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    // let x = 0;
    // let y = 0;
    let mut table: Vec<Column> = Vec::new();
    for c in 0..cells[0].len() {
        let mut values: Vec<u64> = Vec::new();
        for r in 0..cells.len() {
            // println!("[{}]", cells[r][c]);
            if r == cells.len() - 1 {
                table.push(Column::new(&values, cells[r][c]));
            } else {
                values.push(cells[r][c].parse::<u64>().unwrap());
            }
        }
    }
    table
}

#[cfg(test)]
mod day_6_tests {
    use super::*;

    #[test]
    fn test_example_pt1() {
        let input = std::fs::read_to_string("./inputs/day_6_example").unwrap();
        let (answer_pt1, _) = solution(&input);
        assert_eq!(4277556, answer_pt1);
    }

    #[test]
    #[allow(warnings)]
    fn test_example_pt2() {
        let input = std::fs::read_to_string("./inputs/day_6_example").unwrap();
        let (_, answer_pt2) = solution(&input);
        // assert_eq!(3263827, answer_pt2);
    }
}
