use std::fs;

#[derive(PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
    None,
}

struct Command {
    direction: Direction,
    value: i32,
}

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Unable to read file");
    let mut horizontal = 0;
    let mut depth = 0;

    let commands: Vec<Command> = data
        .lines()
        .map(|line| {
            let line_values: Vec<&str> = line.split_whitespace().collect();
            let direction = match line_values[0] {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => Direction::None,
            };
            let value = line_values[1].parse::<i32>().unwrap();

            Command { direction, value }
        })
        .collect();

    for command in commands {
        match command.direction {
            Direction::Forward => horizontal += command.value,
            Direction::Down => depth += command.value,
            Direction::Up => depth -= command.value,
            Direction::None => println!("No such command!"),
        }
    }

    println!("{}", depth * horizontal);
}
