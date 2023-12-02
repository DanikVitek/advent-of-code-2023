use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<usize, AocError> {
    Ok(input.lines().map(process_line).sum())
}

fn process_line(line: &str) -> usize {
    let mut digits = Digits::new(line);
    let first = digits.next().expect("invalid input");
    let last = digits.next_back().unwrap_or(first);

    (first * 10 + last) as usize
}

struct Digits<'a> {
    input: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Digits<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            start: 0,
            end: input.len(),
        }
    }
}

impl<'a> Iterator for Digits<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.start..self.input.len() {
            let Some(digit) = digit_at_start(&self.input[i..]) else {
                continue;
            };
            self.start = i + 1;
            return Some(digit);
        }
        None
    }
}

impl<'a> DoubleEndedIterator for Digits<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        for j in (1..=self.end).rev() {
            let Some(digit) = digit_at_end(&self.input[..j]) else {
                continue;
            };
            self.end = j;
            return Some(digit);
        }
        None
    }
}

const CONVERSIONS: [(&str, u8); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn digit_at_start(input: &str) -> Option<u8> {
    CONVERSIONS
        .into_iter()
        .find_map(|(s, c)| input.starts_with(s).then_some(c))
}

fn digit_at_end(input: &str) -> Option<u8> {
    CONVERSIONS
        .into_iter()
        .find_map(|(s, c)| input.ends_with(s).then_some(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    /// this test case is from the real input
    /// it tests two overlapping numbers
    /// where the second number should succeed
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    fn test_process_line(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process_line(input), expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(process(input)?, 281);
        Ok(())
    }
}
