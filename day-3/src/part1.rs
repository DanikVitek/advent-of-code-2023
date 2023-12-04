use itertools::Itertools;
use nom::{
    bytes::complete::{take_till, take_till1},
    character::complete::digit1,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

pub fn process(input: &str) -> u32 {
    let engine_map: Box<[&str]> = input.lines().collect();

    engine_map
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(y, line)| {
            number_line(line)
                .expect("Invalid input")
                .1
                .into_iter()
                .map(move |num| Number::new(num.as_ptr() as usize - line.as_ptr() as usize, y, num))
        })
        .filter(|num| {
            (num.y.saturating_sub(1)..num.y.wrapping_add(2))
                .cartesian_product(num.x.saturating_sub(1)..num.x + num.value.len().wrapping_add(1))
                .filter(|coords| {
                    let (i, j) = *coords;
                    i < engine_map.len()
                        && j < engine_map[i].len()
                        && (i < num.y || i > num.y || j < num.x || j >= num.x + num.value.len())
                })
                .any(|(i, j)| {
                    let ch = engine_map[i].as_bytes()[j];
                    ch != b'.' && !ch.is_ascii_digit()
                })
        })
        .map(|num| num.value.parse::<u32>().unwrap())
        .sum()
}

#[derive(Debug)]
struct Number<'a> {
    x: usize,
    y: usize,
    value: &'a str,
}

fn number_line(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        take_till(|c: char| c.is_ascii_digit()),
        separated_list0(take_till1(|c: char| c.is_ascii_digit()), digit1),
        take_till(|c: char| "\r\n".contains(c)),
    )(input)
}

impl<'a> Number<'a> {
    fn new(x: usize, y: usize, value: &'a str) -> Self {
        Self { x, y, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, process(input));
    }
}
