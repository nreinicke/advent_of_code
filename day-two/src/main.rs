
#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    quantity: usize,
}

impl Command {
    fn from_line(line: &str) -> Self {
        let split: Vec<&str> = line.split_whitespace().collect();
        let direction_str = split[0];
        let quantity_str = split[1];

        let quantity: usize = quantity_str.parse::<usize>().unwrap(); 

        let direction: Direction = match direction_str {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => Direction::Forward
        }; 
        Command {direction: direction, quantity: quantity}
    }
}

fn apply_commands(commands: Vec<Command>) -> u8 {
    150
}

fn main() {
    let commands: Vec<Command> = include_str!("../input.txt").lines().map(|line| Command::from_line(line)).collect();
    for c in commands {
        println!("{:?}", c.direction);
    }
}


#[test]
fn test_position() {
    let test_commands: Vec<Command> = vec![
        Command { direction: Direction::Forward, quantity: 5 },
        Command { direction: Direction::Down, quantity: 5 },
        Command { direction: Direction::Forward, quantity: 8 },
        Command { direction: Direction::Up, quantity: 3 },
        Command { direction: Direction::Down, quantity: 8 },
        Command { direction: Direction::Forward, quantity: 2 },
    ];
    assert_eq!(apply_commands(test_commands), 150);
}
