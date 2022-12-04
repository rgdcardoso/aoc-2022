use std::fs::File;
use std::io::{prelude::*, BufReader};


fn first() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    
    let mut max_calories = 0;

    let mut buffer = 0;

    for line in reader.lines() {

        let calorie = line.unwrap();

        if calorie.is_empty() {
            if max_calories < buffer {
                max_calories = buffer;
            }
            buffer = 0;
        } else {
            buffer += calorie.parse::<i32>().unwrap();
        }
    }

    println!("Max number of calories: {}", max_calories);
}

fn replace_if_greater(array: &mut [i32], calories: i32) {

    if array.len() == 0 { 
        return;
    };

    let mut buffer = calories;

    if array[0] < calories {
        buffer = array[0];
        array[0] = calories;
    }

    replace_if_greater(&mut array[1 ..], buffer)
}

fn second() {

    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut top_three: [i32;3] = [0;3];

    let mut buffer = 0;

    for line in reader.lines() {

        let calorie = line.unwrap();

        if calorie.is_empty() {
            replace_if_greater(&mut top_three, buffer);
            buffer = 0;
        } else {
            buffer += calorie.parse::<i32>().unwrap();
        }
    }

    let sum_top_three = top_three.into_iter()
        .reduce(|a, b| a + b)
        .unwrap_or(0);

    println!("Sum of top three: {}", sum_top_three);
}

fn main() {

    first();
    second();
}