use std::fs;
use std::process::id;

pub(crate) fn main() {
    let contents = fs::read_to_string("C:/Users/Jakob/Desktop/input")
        .expect("Should have been able to read the file");

    let mut rucksacks: Vec<&str> = contents.split("\n").collect();
    rucksacks.pop();

    let mut matching: Vec<&str> = Vec::new();
    let mut count = 0;

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let mut idx = 0;
    for rucksack in rucksacks.iter().step_by(3) {
        let mut bag1: Vec<&str> = rucksacks[idx].split("").collect();
        bag1.pop();
        bag1.remove(0);

        let mut bag2: Vec<&str> = rucksacks[idx + 1].split("").collect();
        bag2.pop();
        bag2.remove(0);

        let mut bag3: Vec<&str> = rucksacks[idx + 2].split("").collect();
        bag3.pop();
        bag3.remove(0);

        let mut sim: Vec<&str> = Vec::new();

        for a in bag1.iter() {
            for b in bag2.iter() {
                if a == b {
                    for c in bag3.iter() {
                        if b == c {
                            sim.push(c);
                        }
                    }
                }
            }
        }
        matching.push(sim[0]);

        idx += 3;
    }

    for (idx, a) in alphabet.iter().enumerate() {
        for letter in &matching {
            if letter.parse::<char>().unwrap() == alphabet[idx] {
                count += idx + 1;
            }
        }
    }
    println!("{}", count);
}
