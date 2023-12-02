use std::io::BufReader;

use std::fs::File;
use std::io::BufRead;
fn main() {
    let mut calibration_values: Vec<u32> = vec!();
    let mut cal_val:u32 = 0;

    if let Ok(file) = File::open("../input/part1.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines(){
            let mut line_cal_val = String::from("");
            if let Ok(line) = line {
                for char in line.chars(){
                    if char.is_digit(10) {
                        line_cal_val.push(char);
                    }
                }
            }
                let first_digit = line_cal_val.chars().nth(0).unwrap();
                let second_digit = line_cal_val.chars().nth(line_cal_val.len()-1).unwrap();

                let cal_val_num = format!("{}{}", first_digit, second_digit).parse::<u32>().unwrap();
                calibration_values.push(cal_val_num);
        }
    }
    for num in calibration_values.iter(){
        cal_val += num;
    }
    println!("Calibration value: {}", cal_val);
}