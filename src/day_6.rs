use std::collections::HashSet;
use std::ffi::c_char;
use std::fs;

pub(crate) fn main() {
    let contents = fs::read_to_string("C:/Users/Jakob/Desktop/input")
        .expect("Should have been able to read the file");

    let mut signal: Vec<&str> = contents.split("").collect();

    signal.pop();
    signal.remove(0);

    for idx in 0..signal.len() - 14 {
        let mut subtring: String = String::new();

        if idx + 14 < signal.len() {
            for n in 0..14 {
                subtring.push(signal[idx + n].parse().unwrap())
            }
        }

        if is_unique(&*subtring, 0, 14) {
            println!("{}", idx + 14);
            break;
        }
    }
}

fn is_unique(s: &str, start: usize, end: usize) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();

    for i in start..end {
        if !set.insert(chars[i]) {
            return false;
        }
    }
    true
}
