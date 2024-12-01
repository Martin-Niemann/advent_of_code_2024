use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
            let mut result = line.split("   ");
            left_list.push(result.next().unwrap().parse::<u32>().unwrap());
            right_list.push(result.next().unwrap().parse::<u32>().unwrap());
        }
    }

    left_list.sort();
    right_list.sort();

    let mut left_list_iter = left_list.iter();
    let mut right_list_iter = right_list.iter();

    let mut total_distance: u32 = 0;
    while let Some(left_list_number) = left_list_iter.next() {
        let distance = left_list_number.abs_diff(*right_list_iter.next().unwrap());
        total_distance = total_distance + distance;
    }

    println!("{}", total_distance);
}

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
