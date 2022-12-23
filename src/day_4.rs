use std::fs;

pub(crate) fn main() {
    let contents = fs::read_to_string("C:/Users/Jakob/Desktop/input")
        .expect("Should have been able to read the file");

    let mut sections: Vec<&str> = contents.split('\n').collect();
    sections.pop();

    let mut count: i32 = 0;
    for section in sections {
        let sec_per: Vec<&str> = section.split(',').collect();

        let mut last: Vec<i32> = Vec::new();
        for num in sec_per {
            let mut test: Vec<&str> = num.split('-').collect();

            for te in test {
                last.push(te.parse().unwrap())
            }
        }

        if last[1] >= last[2] && last[0] <= last[3] {
            count += 1;
        } else if last[0] <= last[2] && last[1] >= last[3] {
            count += 1;
        } else if last[0] >= last[2] && last[1] <= last[3] {
            count += 1;
        }
    }
    println!("{}", count)
}
