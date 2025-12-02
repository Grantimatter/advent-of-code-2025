fn main() {
    let input =
        std::fs::read_to_string("./inputs/day_1").expect("Should have been able to read the file");
    let zero_clicks = read_instructions(input.lines());
    println!("Zero clicked {zero_clicks} times.")
}

fn read_instructions<'a, I>(instructions: I) -> i32
where
    I: Iterator<Item = &'a str>,
{
    let mut position = 50;
    let mut zero_clicks = 0;

    for rotation in instructions {
        let direction = &rotation[..1];
        let distance = (rotation[1..].trim()).parse::<i32>().unwrap();
        for _ in 0..distance {
            position = turn_dial(position, direction);
            if position == 0 {
                zero_clicks += 1;
            }
        }
    }
    zero_clicks
}

fn turn_dial(starting_point: i32, direction: &str) -> i32 {
    match direction {
        "R" => {
            if starting_point == 99 {
                0
            } else {
                starting_point + 1
            }
        }
        "L" => {
            if starting_point == 0 {
                99
            } else {
                starting_point - 1
            }
        }
        _ => {
            panic!("Unsupported rotation direction")
        }
    }
}
