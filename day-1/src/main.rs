use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn main() {
    let cwd = env::current_dir().expect("Current dir doesn't exist");  
    let path = format!("{}/input/1.txt", cwd.display());

    let file = File::open(path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let increase_count = numbers.windows(2).filter(|w| w[0] < w[1]).count();

    println!("Part 1: {} increased", increase_count);

    let window_sums: Vec<i64> = numbers.windows(3).map(|w| w.iter().sum()).collect();
    let window_increases = window_sums.windows(2).filter(|w| w[0] < w[1]).count();
    println!("Part 2: {} increased", window_increases);
}
