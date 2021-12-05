struct BingoBoard {
    values: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
}

impl BingoBoard {
    fn build(input: &[&str]) -> Self {
        let mut rows: Vec<Vec<usize>> = Vec::new();
        for row in input {
            let cols: Vec<usize> = row
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            rows.push(cols);
        }
        BingoBoard {
            values: rows,
            marked: vec![vec![false; 5]; 5],
        }
    }
}

fn main() {
    println!("Hello, world!");
}

mod test {
    use crate::*;

    fn get_numbers_and_boards() -> (Vec<usize>, Vec<BingoBoard>) {
        let input: Vec<&str> = include_str!("../test_input.txt")
            .lines()
            .filter(|l| l != &"")
            .collect();
        let numbers: Vec<usize> = input[0]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let board_input: Vec<BingoBoard> =
            input[1..].chunks(5).map(|c| BingoBoard::build(c)).collect();

        (numbers, board_input)
    }

    #[test]
    fn test_part_one() {
        let (n, b) = get_numbers_and_boards();
    }

    #[test]
    fn test_part_two() {}
}
