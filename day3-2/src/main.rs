use std::fs;

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Unable to read file");
    let report_data: Vec<&str> = data.lines().collect();

    let oxigen = filter_by_bit(report_data.clone(), true);
    let co2 = filter_by_bit(report_data.clone(), false);

    println!("Result: {}", oxigen * co2);
}

fn most_common_bit(report_data: &Vec<&str>, col: usize) -> String {
    let mut result = String::from("");
    let mut zeros = 0;

    for line in report_data {
        if line.chars().nth(col).unwrap() == '0' {
            zeros += 1;
        }
    }

    if zeros as f32 == report_data.len() as f32 / 2.0 {
        result = "1".to_string();
    } else if zeros as f32 > report_data.len() as f32 / 2.0 {
        result = "0".to_string();
    } else {
        result = "1".to_string();
    }

    result
}

fn least_common_bit(report_data: &Vec<&str>, col: usize) -> String {
    match most_common_bit(&report_data, col).as_ref() {
        "0" => "1".to_string(),
        "1" => "0".to_string(),
        _ => "none".to_string(),
    }
}

fn filter_by_bit(mut elements: Vec<&str>, is_most_common: bool) -> i32 {
    let total_columns = elements.get(0).unwrap().len();

    for col in 0..total_columns {
        if elements.len() == 1 {
            continue;
        };

        let filter_char = match is_most_common {
            true => most_common_bit(&elements, col),
            false => least_common_bit(&elements, col),
        };

        elements.retain(|&value| value.chars().nth(col).unwrap().to_string() == filter_char);
    }

    i32::from_str_radix(&elements.pop().unwrap(), 2).unwrap()
}
