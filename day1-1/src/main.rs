use std::fs;

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Unable to read file");
    let mut previous_number = 10000000;
    let mut result = 1;

    for line in data.lines() {
        let number: u32 = line.parse::<u32>().unwrap();

        if previous_number < number {
            result += 1;
        }
        previous_number = number;
    }

    println!("{}", result);
}
