struct PowerConsumptionDiagnostic {
    message_size: usize,
    counts: Vec<usize>,
    messages_consumed: usize,
}

impl PowerConsumptionDiagnostic {
    fn build(message_size: usize) -> Self {
        PowerConsumptionDiagnostic {
            message_size: message_size,
            counts: vec![0; message_size],
            messages_consumed: 0,
        }
    }

    fn consume_message(&mut self, message: &str) {
        let message_vec: Vec<char> = message.chars().collect();
        for i in 0..self.message_size {
            if message_vec[i] == '1' {
                self.counts[i] += 1;
            }
        }
        self.messages_consumed += 1;
    }

    fn report_power_consumption(self) -> usize {
        let mut gamma_chars: Vec<char> = Vec::new();
        let mut epsilon_chars: Vec<char> = Vec::new();
        for i in 0..self.message_size {
            if self.counts[i] > (self.messages_consumed / 2) {
                gamma_chars.push('1');
                epsilon_chars.push('0');
            } else {
                gamma_chars.push('0');
                epsilon_chars.push('1');
            }
        }
        let gamma_string: String = gamma_chars.into_iter().collect();
        let epsilon_string: String = epsilon_chars.into_iter().collect();

        let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
        let epsilon = usize::from_str_radix(&epsilon_string, 2).unwrap();

        gamma * epsilon
    }
}

// part 2
fn is_char(s: &str, pos: usize, char: char) -> bool {
    let c = s.chars().nth(pos).unwrap();
    c == char
}

fn filter_bits(mut messages: Vec<&str>, message_size: usize, significant_bit: char) -> usize {
    let non_sig_bit = match significant_bit {
        '1' => '0',
        _ => '1'
    };

    for bit_pos in 0..message_size {
        let ones = messages
            .iter()
            .filter(|s| is_char(s, bit_pos, '1'))
            .count();
        let zeros = messages
            .iter()
            .filter(|s| is_char(s, bit_pos, '0'))
            .count();
        if ones >= zeros {
            messages.retain(|s| is_char(s, bit_pos, significant_bit));
        }
        else {
            messages.retain(|s| is_char(s, bit_pos, non_sig_bit));
        }

        if messages.len() == 1 {break;}
    }
    usize::from_str_radix(messages[0], 2).unwrap()
}

fn main() {
    let messages: Vec<&str> = include_str!("../input.txt").lines().collect();
    let message_size: usize = messages[0].len();

    let mut diagnostic = PowerConsumptionDiagnostic::build(message_size);

    for message in messages.clone() {
        diagnostic.consume_message(&message);
    }

    let power_consumption = diagnostic.report_power_consumption();

    println!("Part 1: {}", power_consumption);

    let oxygen = filter_bits(messages.clone(), message_size, '1');
    let co2 = filter_bits(messages.clone(), message_size, '0');
    let life_support = oxygen * co2;

    println!("Part 2: {}", life_support);
}


mod test {
    use crate::*;

    fn get_messages() -> Vec<&'static str> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
    }

    #[test]
    fn test_part_one() {
        let messages = get_messages();
        let mut diagnostic = PowerConsumptionDiagnostic::build(5);

        for message in messages {
            diagnostic.consume_message(&message);
        }

        assert_eq!(diagnostic.report_power_consumption(), 198);
    }

    #[test]
    fn test_part_two() {
        let messages = get_messages();
        let oxygen = filter_bits(messages.clone(), 5, '1');
        let co2 = filter_bits(messages.clone(), 5, '0');
        let life_support = oxygen * co2;
        assert_eq!(life_support, 230);
    }
}
