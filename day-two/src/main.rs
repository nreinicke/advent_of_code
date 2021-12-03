enum Direction {
    Forward,
    Down,
    Up
}

fn direction_from_string(s: &str) -> Result<Direction, &'static str> {
        let direction  = match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err("error building direction") 
        }; 
        direction
}

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

        let direction = direction_from_string(direction_str).unwrap();

        Command {direction: direction, quantity: quantity}
    }
}

fn adjust_position(commands: &Vec<Command>) -> usize {
    let mut depth: usize = 0;
    let mut horizonal: usize = 0;
    for command in commands.iter() {
        match command.direction {
            Direction::Forward => horizonal += command.quantity,
            Direction::Down => depth += command.quantity,
            Direction::Up => depth -= command.quantity
        }
    }
    depth * horizonal
}

fn adjust_position_with_aim(commands: &Vec<Command>) -> usize {
    let mut aim: usize = 0;
    let mut depth: usize = 0;
    let mut horizonal: usize = 0;
    for command in commands.iter() {
        match command.direction {
            Direction::Down => aim += command.quantity,
            Direction::Up => aim -= command.quantity,
            Direction::Forward => {
                horizonal += command.quantity;
                depth += aim * command.quantity;
            }
        }
    }
    depth * horizonal
}

fn main() {
    let commands: Vec<Command> = include_str!("../input.txt").lines().map(|line| Command::from_line(line)).collect();

    let part_1_result = adjust_position(&commands);
    println!("Part 1 Result: {}", part_1_result);

    let part_2_result = adjust_position_with_aim(&commands);
    println!("Part 2 Result: {}", part_2_result);
}

fn get_test_data() -> Vec<Command> {
    let test_commands: Vec<Command> = vec![
        Command { direction: Direction::Forward, quantity: 5 },
        Command { direction: Direction::Down, quantity: 5 },
        Command { direction: Direction::Forward, quantity: 8 },
        Command { direction: Direction::Up, quantity: 3 },
        Command { direction: Direction::Down, quantity: 8 },
        Command { direction: Direction::Forward, quantity: 2 },
    ];
    test_commands
}

#[test]
fn test_part_one() {
    let test_commands = get_test_data();
    assert_eq!(adjust_position(&test_commands), 150);
}

#[test]
fn test_part_two() {
    let test_commands = get_test_data();
    assert_eq!(adjust_position_with_aim(&test_commands), 900);
}
