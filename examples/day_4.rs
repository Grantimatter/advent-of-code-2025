fn main() {
    let input = std::fs::read_to_string("./inputs/day_4").expect("Should be able to read the file");
    let answer_pt1 = solution_pt1(&input);
    let answer_pt2 = solution_pt2(&input);

    println!("Part 1 Answer: {}", answer_pt1);
    println!("Part 2 Answer: {}", answer_pt2);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[allow(warnings)]
fn solution_pt1(input: &str) -> u64 {
    let data = parse_input(input);
    let grid = accessible_rolls(data);
    grid.accessible as u64
}

#[derive(PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct Grid {
    accessible: usize,
    cells: String,
}

fn accessible_rolls(rows: Vec<&str>) -> Grid {
    let mut grid = Grid {
        accessible: 0,
        cells: String::from(""),
    };

    for (idx_row, &row) in rows.iter().enumerate() {
        grid.cells += "\n";
        for (idx_cell, _well) in row.chars().enumerate() {
            let position = Position {
                x: idx_cell,
                y: idx_row,
            };

            let c = get_cell_at_position(&rows, &position);

            if c == 'x' {
                grid.cells += ".";
            } else if can_access_roll(&rows, &position) {
                grid.accessible += 1;
                grid.cells += "x";
            } else {
                grid.cells += &format!("{c}");
            }
        }
    }

    grid
}

fn can_access_roll(rows: &Vec<&str>, position: &Position) -> bool {
    if get_cell_at_position(rows, position) != '@' {
        return false;
    }
    let range_start = Position {
        x: if position.x > 0 {
            position.x - 1
        } else {
            position.x
        },
        y: if position.y > 0 {
            position.y - 1
        } else {
            position.y
        },
    };

    let range_end = Position {
        x: if position.x < rows[0].len() - 1 {
            position.x + 1
        } else {
            position.x
        },
        y: if position.y < rows.len() - 1 {
            position.y + 1
        } else {
            position.y
        },
    };

    let mut adjacent_rolls = 0;
    for x in range_start.x..=range_end.x {
        for y in range_start.y..=range_end.y {
            let current_cell = Position { x, y };
            if &current_cell == position {
                continue;
            }
            if get_cell_at_position(rows, &current_cell) == '@' {
                adjacent_rolls += 1;
                if adjacent_rolls >= 4 {
                    return false;
                }
            }
        }
    }

    true
}

fn get_cell_at_position(rows: &Vec<&str>, position: &Position) -> char {
    rows[position.y].chars().nth(position.x).unwrap()
}

#[allow(warnings)]
fn solution_pt2(input: &str) -> u64 {
    let data = parse_input(input);
    let grid = accessible_rolls(data);
    let mut maximum_rolls = grid.accessible;
    let mut new_grid =
        accessible_rolls(grid.cells.lines().filter(|line| !line.is_empty()).collect());
    loop {
        if new_grid.accessible == 0 {
            break;
        } else {
            maximum_rolls += new_grid.accessible;
        }

        new_grid = accessible_rolls(
            new_grid
                .cells
                .lines()
                .filter(|line| !line.is_empty())
                .collect(),
        );
    }
    maximum_rolls as u64
}

#[cfg(test)]
#[allow(warnings)]
mod tests {
    use super::*;

    #[test]
    #[allow(warnings)]
    fn test_example_pt1() {
        let input = std::fs::read_to_string("./inputs/day_4_example").unwrap();
        println!("\n{input}\n");
        let answer_pt1 = solution_pt1(&input);
        assert_eq!(answer_pt1, 13);
    }

    #[test]
    #[allow(warnings)]
    fn test_example_pt2() {
        let input = std::fs::read_to_string("./inputs/day_4_example").unwrap();
        let answer_pt2 = solution_pt2(&input);
        assert_eq!(answer_pt2, 43);
    }
}
