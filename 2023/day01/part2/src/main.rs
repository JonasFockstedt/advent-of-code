use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut hash = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);
    let mut calibration_values: Vec<u32> = vec!();
    let mut cal_val:u32 = 0;
    let keys: Vec<&str> = hash.map(|s| s.deref()).collect();
    println!("{:?}", keys);

    if let Ok(file) = File::open("../input/part1.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines(){
            let mut str_match = String::from("");
            let mut line_cal_val = String::from("");
            if let Ok(line) = line {
                for line_char in line.chars(){
                    if line_char.is_digit(10) {
                        line_cal_val.push(line_char);
                    }else { // Start looking if we are at a word for a number
                        for number in keys.iter(){
                            for char_num in number.chars(){
                                if char_num == line_char{
                                    str_match.push(char_num); // Found a matching char, add it to match string
                                }else{
                                    str_match = String::from("");
                                    break;
                                }
                            }
                            if str_match == number{
                                line_cal_val.push(hash[str_match]); // Found match, add the corresponding number
                                str_match = String::from("");
                            }
                        }
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
