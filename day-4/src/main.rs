use std::fs;

#[derive(Debug, Clone, Copy)]
struct BingoBoard {
    values: [[usize; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl BingoBoard {
    fn build(input: &[&str]) -> Self {
        let mut values = [[0; 5]; 5];
        for (i, row) in input.iter().enumerate() {
            let cols: Vec<usize> = row
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            for (j, val) in cols.iter().enumerate() {
                values[i][j] = *val;
            }
        }
        BingoBoard {
            values: values,
            marked: [[false; 5]; 5],
        }
    }

    fn play_number(&self, number: usize) -> Self {
        let mut new_marked = self.marked.clone();
        for row in 0..5 {
            for col in 0..5 {
                let board_value: usize = self.values[row][col];
                if board_value == number {
                    new_marked[row][col] = true;

                    // assume there is only one possible play
                    break;
                }
            }
        }
        BingoBoard {
            values: self.values.clone(),
            marked: new_marked,
        }
    }

    fn is_winner(&self) -> bool {
        // check rows
        for row in 0..5 {
            if self.marked[row].iter().filter(|&m| *m).count() == 5 {
                return true;
            }
        }

        // check columns
        for col in 0..5 {
            if self.marked.iter().filter(|&row| row[col]).count() == 5 {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        let mut total_score: usize = 0;
        for row in 0..5 {
            for col in 0..5 {
                if self.marked[row][col] == false {
                    total_score += self.values[row][col];
                }
            }
        }
        total_score
    }
}

fn get_numbers_and_boards(file: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let file_contents = fs::read_to_string(file).unwrap();
    let input: Vec<&str> = file_contents.lines().filter(|l| l != &"").collect();
    let numbers: Vec<usize> = input[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let board_input: Vec<BingoBoard> = input[1..].chunks(5).map(|c| BingoBoard::build(c)).collect();

    (numbers, board_input)
}

fn win_bingo(numbers: Vec<usize>, mut boards: Vec<BingoBoard>) -> usize {
    let mut score = 0;

    'outer: for num in numbers {
        let new_boards: Vec<BingoBoard> = boards.iter().map(|b| b.play_number(num)).collect();
        for b in &new_boards {
            if b.is_winner() {
                score = b.score() * num;
                break 'outer;
            }
        }
        boards = new_boards;
    }
    score
}

fn lose_bingo(numbers: Vec<usize>, mut boards: Vec<BingoBoard>) -> usize {
    let mut score = 0;

    for num in numbers {
        let played_boards: Vec<BingoBoard> = boards.iter().map(|b| b.play_number(num)).collect();

        if played_boards.len() == 1 {
            let last_card = played_boards[0];
            if last_card.is_winner() {
                score = last_card.score() * num;
                break;
            }
        }
        let mut non_winners: Vec<BingoBoard> = Vec::new();
        for b in &played_boards {
            if !b.is_winner() {
                non_winners.push(*b);
            }
        }
        boards = non_winners;
    }
    score
}

fn main() {
    let (numbers, boards) = get_numbers_and_boards(&"input.txt");
    let score = win_bingo(numbers.clone(), boards.clone());
    println!("We have a winner! Score {}", score);
    let score = lose_bingo(numbers, boards);
    println!("We have a loser! Score {}", score);
}

mod test {
    use crate::*;

    #[test]
    fn test_part_one() {
        let (numbers, boards) = get_numbers_and_boards(&"test_input.txt");
        let score = win_bingo(numbers, boards);
        assert_eq!(score, 4512)
    }

    #[test]
    fn test_part_two() {
        let (numbers, boards) = get_numbers_and_boards(&"test_input.txt");
        let score = lose_bingo(numbers, boards);
        assert_eq!(score, 1924)
    }
}
