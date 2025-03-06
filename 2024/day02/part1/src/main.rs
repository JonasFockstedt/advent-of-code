use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let mut safe_reports = 0;
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                // Parse the line into a vector of integers
                reports.push(line.split(" ")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>());
            }
        }
    } else {
        println!("Failed to open the file.");
    }

    for i in 0..reports.len(){
        let mut safe = false; // flag to check if the report is safe
        let temp: Vec<i32> = reports[i].clone(); // clone the report to a temporary vector
        let mut sorted_temp: Vec<i32> = temp.clone(); 
        sorted_temp.sort(); // Sort the temporary vector
        // Check if the report is safe by checking if it is sorted or reverse sorted
        if temp == sorted_temp || temp == sorted_temp.iter().rev().cloned().collect::<Vec<i32>>() {
            for j in 0..temp.len()-1 {
                let diff: i32 = (temp[j] - temp[j+1]).abs();
                if 0 < diff && diff < 4 {
                    // If we are at the last element and the difference has been within 1 and 3, the report is safe
                    if j == temp.len()-2 {
                        safe = true;
                        break;
                    }
                    else {
                        continue;
                    }
                }
                else {
                    safe = false;
                    break;
                }
            }
        } 
        if safe {
            safe_reports += 1;
        } else {
        }
    }
    println!("Number of safe reports: {}", safe_reports);
}
