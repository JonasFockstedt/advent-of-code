use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    // Store the columns in two vectors and hashmaps to keep track of the count of each element for each column
    let mut first_col: Vec<i32> = Vec::new();
    let mut first_hash: HashMap<i32,i32>  = HashMap::new();
    let mut sec_col: Vec<i32> = Vec::new();
    let mut sec_hash: HashMap<i32,i32>  = HashMap::new();

    // Get the command line arguments
    let arg: Vec<String> = env::args().collect();
    if let Ok(lines) = read_lines(arg[1].to_string()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let line_arr: Vec<&str> = line.split(" ").collect();
            if !line_arr.is_empty() {
                // Check if we can parse the first elemet of the line to an integer
                if let Ok(num) = line_arr[0].parse::<i32>() {
                    first_col.push(num);
                    // Check if the element is already in the hashmap
                    if let Some(count) = first_hash.get(&num) {
                        first_hash.insert(num, count+1);
                    } else {
                        first_hash.insert(num, 1);
                    }
                }
                // Check if we can parse the last elemet of the line to an integer
                if let Ok(num) = line_arr[line_arr.len()-1].parse::<i32>() {
                    sec_col.push(num);
                    // Check if the element is already in the hashmap
                    if let Some(count) = sec_hash.get(&num) {
                        sec_hash.insert(num, count+1);
                    } else {
                        sec_hash.insert(num, 1);
                    }
                }
            }
        }
    }
    first_col.sort(); 
    sec_col.sort();
    // Calculate the distance between the two columns
    let distances: Vec<u32> = first_col.iter().zip(sec_col.iter()).map(|(a, b)| a.abs_diff(*b)).collect();

    let total_distance: u32 = distances.iter().sum();
    let sim_score: i32 = calc_similarity_score(first_col, sec_hash);
    println!("Total distance: {:?}", total_distance);
    println!("Similarity score: {:?}", sim_score);

}

// Calculate the similarity score between the two columns
// The score is the sum of the product of the count of each element in the first column and the element in the second column
fn calc_similarity_score(col: Vec<i32>, hash: HashMap<i32,i32>) -> i32 {
    let mut score: i32 = 0;
    for num in col {
        if let Some(count) = hash.get(&num) {
            score += count*num;
        }
    }
    return score;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


