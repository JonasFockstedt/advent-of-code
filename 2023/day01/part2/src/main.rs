use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

fn main() {
    let hash = HashMap::from([
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
    let mut lines = Vec::new();
    let file_path = "../input/part1.txt";

    // Transfer each line of the input into an index in a vector
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        lines.push(line.unwrap());
    }


    let mut calibration_values: Vec<i32> = Vec::new();
    let mut cal_val:i32 = 0;

    
    for line in lines.iter(){
        let mut found_vals = Vec::new(); // vector of tuples of which number was found where
        for key in hash.keys(){
            let num_char_index = line.find(key);
            let num_index = line.find(hash[key]);

            // (number, found at index) <- structure of tuple
            if let Some(num_char_index) = num_char_index{
                let number = hash[key];
                found_vals.push((number, num_char_index));

            }
            if let Some(num_index) = num_index{
                found_vals.push((hash[key], num_index));
            }
        }
        // sort the vector based on index 1 of the tuples, which is the index the number was found
        found_vals.sort_by_key(|k| k.1);
        let (first_digit, _) = found_vals[0];
        let (last_digit, _) = found_vals[found_vals.len()-1];
        let cal_val = format!("{}{}",first_digit,last_digit).parse::<i32>().unwrap();
        calibration_values.push(cal_val);

    }
    for num in calibration_values.iter(){
        cal_val += num;
    }
    println!("Calibration value: {}", cal_val);
}
