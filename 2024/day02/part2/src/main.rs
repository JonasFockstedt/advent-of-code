use std::env;
use std::fs::File;
use std::io::{self, BufRead};

// Function to check if a report is safe
fn is_report_safe(levels: &[i32]) -> bool {
    if levels.len() <= 1 {
        return true; // A report with 0 or 1 level is trivially safe
    }

    let mut is_valid_ascending = true;
    let mut is_valid_descending = true;

    for j in 0..levels.len() - 1 {
        let current = levels[j];
        let next = levels[j + 1];

        // Check ascending
        let asc_diff = next - current;
        if asc_diff < 1 || asc_diff > 3 {
            is_valid_ascending = false;
        }

        // Check descending
        let desc_diff = current - next;
        if desc_diff < 1 || desc_diff > 3 {
            is_valid_descending = false;
        }

        // Early termination if neither condition can be met
        if !is_valid_ascending && !is_valid_descending {
            return false;
        }
    }

    is_valid_ascending || is_valid_descending
}

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
                reports.push(
                    line.split(" ")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                );
            }
        }
    } else {
        println!("Failed to open the file.");
    }

    for report in &reports {
        println!("Analyzing row: {:?}", report);

        // First check if the report is already safe
        if is_report_safe(report) {
            safe_reports += 1;
            println!("  --> SAFE (without removing any level)");
            continue;
        }

        // If not safe, try removing each level and check if it becomes safe
        let mut can_be_made_safe = false;
        for i in 0..report.len() {
            // Create a new vector without the element at index i
            let mut modified_report = report.clone();
            modified_report.remove(i);

            if is_report_safe(&modified_report) {
                can_be_made_safe = true;
                println!("  --> SAFE (by removing level at index {})", i);
                break;
            }
        }

        if can_be_made_safe {
            safe_reports += 1;
        } else {
            println!("  --> UNSAFE");
        }
    }

    println!("Number of safe reports: {}", safe_reports);
}
