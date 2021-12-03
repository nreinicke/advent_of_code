struct PowerConsumptionDiagnostic {
    message_size: usize,
    counts: Vec<usize>,
    messages_consumed: usize
}

impl PowerConsumptionDiagnostic {
    fn build(message_size: usize) -> Self {
        PowerConsumptionDiagnostic {
            message_size: message_size,
            counts: vec![0; message_size],
            messages_consumed: 0
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
            }
            else {
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

fn main() {
    let messages: Vec<&str> = include_str!("../input.txt").lines().collect();

    let mut diagnostic = PowerConsumptionDiagnostic::build(12);

    for message in messages {
        diagnostic.consume_message(&message);
    }

    let power_consumption = diagnostic.report_power_consumption();

    println!("Part 1: {}", power_consumption);
}

mod test {
    use crate::*;

    fn get_test_data() -> Vec<&'static str> {
        vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ]
        
    }

    #[test]
    fn test_part_one() {
        let test_data = get_test_data();
        let mut diagnostic = PowerConsumptionDiagnostic::build(5);

        for message in test_data {
            diagnostic.consume_message(&message);
        }

        assert_eq!(diagnostic.report_power_consumption(), 198);
    }

    #[test]
    fn test_part_two() {}
}
