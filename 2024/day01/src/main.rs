use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut first_col: Vec<i32> = Vec::new();
    let mut sec_col: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./input/part1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let line_arr: Vec<&str> = line.split(" ").collect();
            if !line_arr.is_empty() {
                if let Ok(num) = line_arr[0].parse::<i32>() {
                    first_col.push(num);
                }
                if let Ok(num) = line_arr[line_arr.len()-1].parse::<i32>() {
                    sec_col.push(num);
                }
            }
            // println!("{:?}", line_arr);
        }
    }
    first_col.sort(); 
    sec_col.sort();
    let distances: Vec<u32> = first_col.iter().zip(sec_col.iter()).map(|(a, b)| a.abs_diff(*b)).collect();
    let total_distance: u32 = distances.iter().sum();
    println!("{:?}", first_col);
    println!("{:?}", sec_col);
    println!("{:?}", distances);
    println!("{:?}", total_distance);

}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


