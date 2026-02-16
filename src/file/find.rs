use std::fs::File;
use std::io::{BufRead, BufReader};
use log::{debug, error, info};

pub fn find_by_line_number(file_location: String,
                                line_number: usize,
                                lines_before: usize,
                                lines_after: usize) -> Vec<(usize, String)> {
    // line number should be 1-based. Only 0-based when interacting with file.
    if line_number == 0 {
        error!("line number cannot be 0");
        return Vec::new();
    }

    let file = match File::open(&file_location) {
        Ok(file) => file,
        Err(_) => {
            error!("failed to open file {}", &file_location);
            return Vec::new();
        },
    };

    // Turning the 1-based line number into 0-based values.
    let fixed_line_number = line_number.saturating_sub(1);

    let mut result_lines: Vec<(usize, String)> = Vec::new();
    let file_lines = BufReader::new(&file).lines();
    let first_line_number = fixed_line_number.saturating_sub(lines_before);

    for line in file_lines.enumerate().skip(first_line_number.saturating_sub(1)) {
        if let Ok(text) = line.1 {
            if line.0 >= fixed_line_number.saturating_sub(lines_before) &&
                line.0 <= fixed_line_number.saturating_add(lines_after) {
                result_lines.push((line.0.saturating_add(1), text.clone()))
            }
            if line.0 > fixed_line_number.saturating_add(lines_after) {
                break;
            }
        }
    }
    result_lines
}

// #[cfg(test)]
mod tests {
    use std::time::{Instant};
    use super::*;

    // #[test]
    // Taking 2.8µs to execute.
    fn test_find_by_line_number_zero() {
        let start = Instant::now();
        let result = find_by_line_number("./test-logs/numbers-hundred-milion.log".to_string(), 0, 0, 3);
        let end = Instant::now();
        println!("execution took: {:?}", end.duration_since(start));
        assert_eq!(result.len(), 0);
    }

    // #[test]
    // Taking 100.2µs to execute.
    fn test_find_by_line_number_init_multiple() {
        let start = Instant::now();
        let result = find_by_line_number("./test-logs/numbers-hundred-milion.log".to_string(), 1, 0, 3);
        let end = Instant::now();
        println!("execution took: {:?}", end.duration_since(start));
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], (1, "minus-fifty-milion: menos cinquenta milhoes - -50000000".to_string()));
        assert_eq!(result[1], (2, "minus-forty-nine-milion-nine-hundred-ninety-nine\
        -thousand-nine-hundred-ninety-nine: menos quarenta e nove milhoes e novecentos \
        e noventa e nove mil e novecentos e noventa e nove - -49999999".to_string()));
        assert_eq!(result[2], (3, "minus-forty-nine-milion-nine-hundred-ninety-nine\
        -thousand-nine-hundred-ninety-eight: menos quarenta e nove milhoes e novecentos \
        e noventa e nove mil e novecentos e noventa e oito - -49999998".to_string()));
        assert_eq!(result[3], (4, "minus-forty-nine-milion-nine-hundred-ninety-nine\
        -thousand-nine-hundred-ninety-seven: menos quarenta e nove milhoes e novecentos \
        e noventa e nove mil e novecentos e noventa e sete - -49999997".to_string()));
    }

    // #[test]
    // Taking 25.708569s to execute.
    fn test_find_by_line_number_middle_multiple() {
        let start  = Instant::now();
        let result = find_by_line_number(
            "./test-logs/numbers-hundred-milion.log".to_string(), 46387545, 3, 3);
        let end = Instant::now();
        println!("execution took: {:?}", end.duration_since(start));
        assert_eq!(result.len(), 7);
        assert_eq!(result[0], (46387542, "minus-three-milion-six-hundred-twelve-thousand-four-\
        hundred-fifty-nine: menos tres milhoes e seiscentos e doze mil e quatrocentos e cinquenta \
        e nove - -3612459".to_string()));
        assert_eq!(result[1], (46387543, "minus-three-milion-six-hundred-twelve-thousand-four-\
        hundred-fifty-eight: menos tres milhoes e seiscentos e doze mil e quatrocentos e cinquenta \
        e oito - -3612458".to_string()));
        assert_eq!(result[2], (46387544, "minus-three-milion-six-hundred-twelve-thousand-four-\
        hundred-fifty-seven-prime: menos tres milhoes e seiscentos e doze mil e quatrocentos \
        e cinquenta e sete numero primo - -3612457".to_string()));
        assert_eq!(result[3], (46387545, "minus-three-milion-six-hundred-twelve-thousand-four-\
        hundred-fifty-six: menos tres milhoes e seiscentos e doze mil e quatrocentos e cinquenta \
        e seis - -3612456".to_string()));
        assert_eq!(result[4], (46387546, "minus-three-milion-six-hundred-twelve-thousand-four-\
        hundred-fifty-five: menos tres milhoes e seiscentos e doze mil e quatrocentos e cinquenta \
        e cinco - -3612455".to_string()));
        assert_eq!(result[5], (46387547, "minus-three-milion-six-hundred-twelve-thousand-four-\
        hundred-fifty-four: menos tres milhoes e seiscentos e doze mil e quatrocentos e cinquenta \
        e quatro - -3612454".to_string()));
        assert_eq!(result[6], (46387548, "minus-three-milion-six-hundred-twelve-thousand-four-\
        hundred-fifty-three: menos tres milhoes e seiscentos e doze mil e quatrocentos e cinquenta \
        e tres - -3612453".to_string()));
    }

    // #[test]
    // Taking 52.6357405s to execute.
    fn test_find_by_line_number_end_multiple() {
        let start  = Instant::now();
        let result = find_by_line_number("./test-logs/numbers-hundred-milion.log".to_string(), 98522240, 100, 100);
        let end = Instant::now();
        println!("execution took: {:?}", end.duration_since(start));
        assert_eq!(result.len(), 201);
        assert_eq!(result[0], (98522140, "plus-forty-eight-milion-five-hundred-twenty-two-thousand-one-hundred-thirty-nine-prime: mais quarenta e oito milhoes e quinhentos e vinte e dois mil e cento e trinta e nove numero primo - 48522139".to_string()));
        assert_eq!(result[100], (98522240, "plus-forty-eight-milion-five-hundred-twenty-two-thousand-two-hundred-thirty-nine: mais quarenta e oito milhoes e quinhentos e vinte e dois mil e duzentos e trinta e nove - 48522239".to_string()));
        assert_eq!(result[200], (98522340, "plus-forty-eight-milion-five-hundred-twenty-two-thousand-three-hundred-thirty-nine: mais quarenta e oito milhoes e quinhentos e vinte e dois mil e trezentos e trinta e nove - 48522339".to_string()));
    }
}