use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    let data = load_data_to_vector("input.txt");
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<i32>) {
    let data_len = data.len();
    let mut increases = 0;

    for i in 1..data_len as usize {
        let curr = data[i];
        let prev = data[i - 1];

        match curr.cmp(&prev) {
            Ordering::Greater => increases += 1,
            Ordering::Less => (),
            Ordering::Equal => (),
        }
    }

    println!("Number of increasing measurements {}", increases);
}

fn part_2(data: &Vec<i32>) {
    let data_len = data.len();
    let mut window_increases = 0;

    for i in 3..data_len as usize {
        let prev_three_sum = data[i - 1] + data[i - 2] + data[i - 3];
        let three_sum = data[i] + data[i - 1] + data[i - 2];

        match three_sum.cmp(&prev_three_sum) {
            Ordering::Greater => window_increases += 1,
            Ordering::Less => (),
            Ordering::Equal => (),
        }
    }

    println!(
        "Number of window increasing measurements {}",
        window_increases
    );
}

fn load_data_to_vector(filename: &str) -> Vec<i32> {
    let mut data: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let ip_int = ip.parse::<i32>().unwrap();
                data.push(ip_int);
            }
        }
    }
    return data;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
