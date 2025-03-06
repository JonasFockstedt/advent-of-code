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

    for i in 0..reports.len() {
        let temp: Vec<i32> = reports[i].clone(); // clone the report to a temporary vector
        
        // Check if sequence follows the rules with at most one violation
        let mut ascending_violations = 0;
        let mut descending_violations = 0;
        
        // Check ascending order
        for j in 0..temp.len()-1 {
            let diff = temp[j+1] - temp[j];
            if diff <= 0 || diff > 3 {
                ascending_violations += 1;
            }
        }
        
        // Check descending order
        for j in 0..temp.len()-1 {
            let diff = temp[j] - temp[j+1];
            if diff <= 0 || diff > 3 {
                descending_violations += 1;
            }
        }
        
        // Report is safe if either direction has at most one violation
        if ascending_violations <= 1 || descending_violations <= 1 {
            safe_reports += 1;
        }
    }
    println!("Number of safe reports: {}", safe_reports);
}