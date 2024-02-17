use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let bag = HashMap::from([("blue", 14), ("red", 12), ("green", 13)]);
    let mut possible_game_ids: Vec<u16> = Vec::new();

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
        // "Game 1" -> "1" -> 1
        let game_id: Vec<u16> = splitted
            .iter()
            .map(|x| x.split(" "))
            .filter(|y| y.clone().count() == 2)
            .map(|z| z.clone().nth(1).unwrap().parse::<u16>().unwrap())
            .collect();
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
            let mut possible_counter: i8 = 0;
            if i == games.len() - 1 {
                for (color, amount) in bag.iter() {
                    if cubes.contains_key(color) {
                        // Check if the amount of the given color in the bag is greater than or equal to what was showed during the game
                        if amount >= &cubes[color] {
                            possible_counter = &possible_counter + 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                    if possible_counter == 3 {
                        possible_game_ids.push(game_id[0]);
                    }
                }
            }
        }
    }
    let sum: u16 = possible_game_ids.iter().sum();
    println!("Sum of possible games: {}", sum);
}
