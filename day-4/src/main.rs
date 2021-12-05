use std::fs;

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

    fn play_number(&mut self, number: usize) {
        for row in 0..5 {
            for col in 0..5 {
                let board_value: usize = self.values[row][col]; 
                if board_value == number {
                    self.marked[row][col] = true;

                    // assume there is only one possible play
                    break;
                }
            }
        }
    }

    fn is_winner(self) -> bool {
        // check rows
        for row in 0..5 {
            if self.marked[row].iter().filter(|&m| *m).count() == 5 {
                return true
            }
        }

        // check columns
        for col in 0..5 {
            if self.marked.iter().filter(|&row| row[col]).count() == 5 {
                return true
            }
        }
        false
    }
}

fn get_numbers_and_boards(file: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let file_contents = fs::read_to_string(file).unwrap();
    let input: Vec<&str> = file_contents 
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

fn main() {
    println!("Hello, world!");
}

mod test {
    use crate::*;


    #[test]
    fn test_part_one() {
        let (numbers, boards) = get_numbers_and_boards(&"test_input.txt");
        for mut b in boards {
            b.play_number(22);
            b.is_winner();
        }
    }

    #[test]
    fn test_part_two() {}
}
