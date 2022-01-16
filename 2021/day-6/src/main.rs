use std::fs;

const SIM_DAYS: usize = 256;

fn count(values: &Vec<u8>, query: u8) -> usize {
    values.iter().filter(|&n| *n == query).count()
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let fish: Vec<u8> = file_contents
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let mut fish_counts: Vec<usize> = (0..9).map(|i| count(&fish, i)).collect();
    for day in 0..SIM_DAYS {
        fish_counts[(day + 7) % 9] += fish_counts[day % 9];
    }
    let total_fish: usize = fish_counts.iter().sum(); 
    println!("{total_fish:?}");
}
