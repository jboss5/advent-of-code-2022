use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Error opening input file");
    let reader = BufReader::new(file);
    let mut calories: Vec<i32> = Vec::new();
    let mut index = 0;
    calories.push(0);

    for l in reader.lines() {
        let line = &l.unwrap();
        if line != "" {
            match line.parse::<i32>() {
                Ok(n) => calories[index] = n + calories[index],
                Err(e) => eprintln!("found non-number in input: {}", e),
            }
        } else {
            index += 1;
            calories.push(0);
        }
    }

    calories.sort_by(|a,b| b.cmp(a));
    let max = calories.iter().max().expect("Internal error");
    let top3sum = calories[0] + calories[1] + calories[2];
    println!("highest calorie count: {}", max);
    println!("top 3 sum: {}", top3sum);
}
