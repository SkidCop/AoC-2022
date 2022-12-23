use std::fs;

pub(crate) fn main() {
    let contents = fs::read_to_string("C:/Users/Jakob/Desktop/input")
        .expect("Should have been able to read the file");

    let mut lines: Vec<&str> = contents.split('\n').collect();

    lines.pop();

    let mut count: i32 = 0;

    for line in &lines {
        let games: Vec<&str> = line.split(' ').collect();

        match games[0] {
            "A" => match games[1] {
                "X" => count += 3,
                "Y" => count += 4,
                "Z" => count += 8,
                _ => {}
            },
            "B" => match games[1] {
                "X" => count += 1,
                "Y" => count += 5,
                "Z" => count += 9,
                _ => {}
            },
            "C" => match games[1] {
                "X" => count += 2,
                "Y" => count += 6,
                "Z" => count += 7,
                _ => {}
            },
            _ => {}
        }
    }
    println!("{}", count);
}
