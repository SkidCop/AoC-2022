use std::fs;

pub(crate) fn main() {
    let contents = fs::read_to_string("C:/Users/Jakob/Desktop/input")
        .expect("Should have been able to read the file");

    let cal_total: Vec<&str> = contents.split("\n").collect();

    let mut cal_per: Vec<i32> = Vec::new();

    let mut cal_sum: Vec<i32> = Vec::new();

    for cal in cal_total {
        if !cal.is_empty() {
            cal_per.push(cal.parse().unwrap());
        } else {
            cal_sum.push(cal_per.iter().sum());
            cal_per.clear();
        }
    }

    cal_sum.sort();

    let len = cal_sum.len();
    let sum = cal_sum[len - 1] + cal_sum[len - 2] + cal_sum[len - 3];

    println!("{}", sum)
}
