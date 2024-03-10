use std::io;

// On each line, the calibration value can be found by combining the first
// digit and the last digit (in that order) to form a single two-digit number.
// finally, add all the values together

// const PUZZLE_INPUT: &str = "
// ";
// const PUZZLE_INPUT: &str = "
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// ";

const RADIX: u32 = 10;

fn main() {
    let mut values: Vec<i32> = Vec::new();
    // for (_, line) in PUZZLE_INPUT.lines().enumerate() {
    for wrapped in io::stdin().lines() {
        let line = wrapped.unwrap();
        if line.is_empty() {
            continue;
        }

        let mut first_num: u32 = 0;
        let mut last_num: u32 = 0;
        let mut first_set: bool = false;
        for c in line.chars() {
            let test = c.to_digit(RADIX);
            if test == None {
                continue;
            }

            last_num = test.unwrap();
            if !first_set {
                first_set = true;
                first_num = test.unwrap();
            }
        }

        // Need to do something like `${first_num}${last_num}`
        let value = format!("{first_num}{last_num}");
        let value_num = value.parse::<i32>().unwrap();
        values.push(value_num);
    }

    let sum: i32 = values.iter().sum();
    println!("Sum: {}", sum)
}
