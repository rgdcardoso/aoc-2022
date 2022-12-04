use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;

fn parse_rucksack(row: &str) -> (Vec<char>, Vec<char>) {

    let rucksack: Vec<_> = row.chars().collect();

    let first_compartment = rucksack[0 .. (row.len() / 2)].to_vec();
    let second_compartment = rucksack[(row.len() / 2) ..].to_vec();

    (first_compartment, second_compartment)
}

fn get_item_priority(item: char) -> usize {

    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    chars.iter().position(|&c| c == item).unwrap() + 1
}

fn first() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut total_priority: usize = 0;

    for line in reader.lines() {

        let line = line.unwrap();

        let rucksack = line.as_str();

        let (first, second) = parse_rucksack(rucksack);

        let comparator: HashSet<char> = HashSet::from_iter(second.iter().cloned());
        
        for item in first {

            if comparator.contains(&item) {
                total_priority += get_item_priority(item);
                break;
            }

        }   
    }

    println!("Total priority {}", total_priority);

}

fn get_badge(group: Vec<HashSet<char>>) -> char {
    group.into_iter()
        .reduce(|a, b| -> HashSet<char> {
            HashSet::from_iter(a.intersection(&b)
                .into_iter()
                .map(|c| *c))
            }
        )
        .unwrap()
        .into_iter()
        .next()
        .unwrap()      
}

fn second() {

    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut total_priority: usize = 0;

    let mut group: Vec<HashSet<char>> = Vec::new();

    for line in reader.lines() {

        let rucksack = line.unwrap();

        if group.len() == 3 {
            
            let badge = get_badge(group.clone());

            total_priority += get_item_priority(badge);
            group.clear();
            group.push(HashSet::from_iter(rucksack.chars()));
        } else {
            group.push(HashSet::from_iter(rucksack.chars()));
        }
    }

    let badge = get_badge(group.clone());
    total_priority += get_item_priority(badge);

    println!("Total priority {}", total_priority);

}

fn main() {
    first();
    second();
}
