use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value, IResult,
};

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<usize, AocError> {
    Ok(input.lines().map(process_line).sum())
}

fn process_line(line: &str) -> usize {
    let mut digits = Digits::new(line);
    let first = digits.next().expect("invalid input");
    let last = digits.last().unwrap_or(first);

    (first * 10 + last) as usize
}

struct Digits<'a> {
    input: &'a str,
}

impl<'a> Digits<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Iterator for Digits<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.input.len() {
            let Ok((_, digit)) = digit(&self.input[i..]) else {
                continue;
            };
            self.input = &self.input[i + 1..];
            return Some(digit);
        }
        None
    }
}

fn spelled_digit(input: &str) -> IResult<&str, u8> {
    alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

fn ascii_digit(input: &str) -> IResult<&str, u8> {
    alt((
        value(1, char('1')),
        value(2, char('2')),
        value(3, char('3')),
        value(4, char('4')),
        value(5, char('5')),
        value(6, char('6')),
        value(7, char('7')),
        value(8, char('8')),
        value(9, char('9')),
    ))(input)
}

fn digit(input: &str) -> IResult<&str, u8> {
    alt((ascii_digit, spelled_digit))(input)
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
