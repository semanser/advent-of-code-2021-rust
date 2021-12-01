use std::cmp;
use std::fs;

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Unable to read file");
    let mut previous_sum = 10000000;
    let mut result = 1;

    let numbers: Vec<u32> = data
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    for index in 0..numbers.len() {
        let ref slice = &numbers[index..cmp::min(index + 3, numbers.len())];
        let sum: u32 = slice.iter().sum();

        if previous_sum < sum {
            result += 1;
        }
        previous_sum = sum;
    }
    println!("{}", result);
}
