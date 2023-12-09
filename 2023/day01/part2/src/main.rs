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
            // (number, found at index) <- structure of tuple
            // for finding written numbers
            let num_str_indicies: Vec<_> = line.match_indices(key).map(|(i, _)| i).collect(); // returns all indicies of the number in words
            if num_str_indicies.len() > 0{
                for index in 0..num_str_indicies.len(){
                    found_vals.push((hash[key], num_str_indicies[index]))
                }
            }
            // for finding numbers
            let num_indicies: Vec<_> = line.match_indices(hash[key]).map(|(i, _)| i).collect(); // returns all indicies of the number
            if num_indicies.len() > 0{
                for index in 0..num_indicies.len(){
                    found_vals.push((hash[key], num_indicies[index]))
                }
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
