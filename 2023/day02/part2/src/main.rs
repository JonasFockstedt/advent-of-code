use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut min_possible: Vec<i32> = Vec::new();

    let file_path = "../input/part1.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    for line in lines.iter() {
        // (color, amount)
        let mut cubes: HashMap<&str, i32> = HashMap::new();
        let splitted: Vec<&str> = line.split(":").collect();
        // "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" -> [3 blue, 4 red] [1 red, 2 green, 6 blue] [2 green]
        let games: Vec<String> = splitted[1]
            .split(";")
            .map(|x| x.split(",").collect::<String>())
            .into_iter()
            .map(|y| y.chars().skip(1).collect::<String>())
            .collect();
        for i in 0..games.len() {
            let set: Vec<&str> = games[i].split_whitespace().collect();
            for j in 0..set.len() {
                if j % 2 == 0 {
                    let color = set[j + 1];
                    let amount = set[j].parse::<i32>().unwrap();
                    // println!("{} {:?}", color, amount);
                    if cubes.contains_key(color) {
                        if cubes[color] < amount {
                            cubes.insert(color, amount);
                        }
                    } else {
                        cubes.entry(&color).or_insert(amount);
                    }
                }
            }
            // If we're at the last game line of the file
            if i == games.len() - 1 {
                let mut least: Vec<i32> = Vec::new();
                for key in cubes.keys() {
                    least.push(cubes[key]);
                }
                min_possible.push(least.iter().product());
            }
        }
    }
    let total_sum: i32 = min_possible.iter().sum();
    println!("Total sum: {:?}", total_sum);
}
