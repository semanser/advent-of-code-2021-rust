use std::fs;

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Unable to read file");
    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    let report_data: Vec<&str> = data.lines().collect();
    let number_of_columns = report_data[0].len();

    let mut number_of_zeros_per_column = vec![0; number_of_columns];

    for i in 0..number_of_columns {
        for line in &report_data {
            if line.chars().nth(i).unwrap() == '0' {
                number_of_zeros_per_column[i] += 1;
            }
        }
    }

    for zeros in &number_of_zeros_per_column {
        if zeros > &500 {
            gamma.push_str("0");
            epsilon.push_str("1");
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }

    let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", gamma_int * epsilon_int);
}
