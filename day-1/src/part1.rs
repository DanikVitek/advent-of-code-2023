pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(char_to_digit);
            let first = digits.next().expect("invalid input");
            let last = digits.next_back().unwrap_or(first);

            (first * 10 + last) as u32
        })
        .sum()
}

fn char_to_digit(ch: char) -> Option<u8> {
    match ch {
        '0'..='9' => Some(ch as u8 - b'0'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(process(input), 142);
    }
}
