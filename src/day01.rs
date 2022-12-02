use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldn't find file.");

    let mut running_sum = 0;
    let mut cals = Vec::new();
    for line in input.lines() {
        if line == "" {
            cals.push(running_sum);
            running_sum = 0;
        } else {
            let cals_in_line = line.parse::<i32>().unwrap();
            running_sum += cals_in_line;
        }
    }

    let mut first = i32::MIN;
    let mut second = i32::MIN;
    let mut third = i32::MIN;
    for cal in cals {
        if cal > first {
            third = second;
            second = first;
            first = cal;
        } else if cal > second {
            third = second;
            second = cal;
        } else if cal > third {
            third = cal;
        }
    }

    println!("{}", first + second + third);
}
