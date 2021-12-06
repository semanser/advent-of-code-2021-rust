use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Unable to read file");
    let amount_of_days = 80;
    let mut result = 0;

    // Read all the input data
    let mut timers: HashMap<u16, i64> = HashMap::new();
    data.split(',').for_each(|number| {
        let value = number.trim().parse::<u16>().unwrap();

        if timers.contains_key(&value) {
            let old_counter = timers.get(&value).unwrap().clone();
            timers.insert(value, old_counter + 1);
        } else {
            timers.insert(value, 1);
        }
    });

    println!("Initial - {:?}", timers);

    for i in 0..amount_of_days {
        let mut timers_new: HashMap<u16, i64> = HashMap::new();
        let mut keys: Vec<u16> = timers.keys().cloned().collect();
        keys.sort();
        keys.reverse();

        for day in keys {
            if day as i32 == 0 {
                // Add a new fish
                let counter = timers.get(&day).unwrap().clone();
                timers_new.insert(8, counter);

                // Reset internal timer
                if timers_new.contains_key(&6) {
                    let old_value = timers_new.get(&6).unwrap().clone();
                    let value = timers.get(&day).unwrap().clone();
                    timers_new.insert(6, old_value + value);
                } else {
                    let value = timers.get(&day).unwrap().clone();
                    timers_new.insert(6, value);
                }
            } else {
                let value = timers.get(&day).unwrap().clone();
                timers_new.insert(day - 1, value);
            }
        }
        timers = timers_new.clone();

        // println!("After day #{} - {:?}", i + 1, timers);
    }
    result = timers.values().sum();

    println!("Result: {:?}", result);
}
