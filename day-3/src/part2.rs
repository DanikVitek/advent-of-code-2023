use itertools::Itertools;
use nom::character::complete::digit1;

pub fn process(input: &str) -> u32 {
    let engine_map: Box<[&str]> = input.lines().collect();

    engine_map
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter_map(move |(x, ch)| (ch == '*').then_some((x, y)))
        })
        .inspect(
            #[cfg(debug_assertions)]
            {
                |(x, y)| eprintln!("{}: (x: {}, y: {})", line!(), x, y)
            },
            #[cfg(not(debug_assertions))]
            {
                |_| ()
            },
        )
        .filter_map(|(x, y)| {
            let mut visited_coords = Vec::with_capacity(8);
            let adjacent_part_numbers = (y.saturating_sub(1)..=y.wrapping_add(1))
                .cartesian_product((x.saturating_sub(1)..=x.wrapping_add(1)).rev())
                // .inspect(|(i, j)| eprintln!("{}: (i: {}, j: {})", line!(), i, j))
                .filter(|coords| {
                    let (i, j) = *coords;
                    i < engine_map.len()
                        && j < engine_map[i].len()
                        && (i < y || i > y || j < x || j > x)
                })
                // .inspect(|(i, j)| eprintln!("{}: (i: {}, j: {}): {:?}", line!(), i, j, engine_map[*i].as_bytes()[*j] as char))
                .fold(Vec::new(), |mut adjacent_part_numbers, (i, j)| {
                    if visited_coords.contains(&(i, j)) {
                        return adjacent_part_numbers;
                    }
                    let ch = engine_map[i].as_bytes()[j];
                    if ch.is_ascii_digit() {
                        #[cfg(debug_assertions)]
                        eprintln!("{}: (i: {}, j: {}): {:?}", line!(), i, j, ch as char);
                        let mut start = j;
                        while start > 0 && engine_map[i].as_bytes()[start - 1].is_ascii_digit() {
                            start -= 1;
                            visited_coords.push((i, start));
                        }
                        let part_number = digit1::<_, ()>(&engine_map[i][start..])
                            .unwrap()
                            .1
                            .parse::<u32>()
                            .unwrap();
                        #[cfg(debug_assertions)]
                        eprintln!("{}: {}", line!(), part_number);
                        adjacent_part_numbers.push(part_number);
                    }

                    visited_coords.push((i, j));
                    adjacent_part_numbers
                });
            (adjacent_part_numbers.len() == 2)
                .then(|| adjacent_part_numbers.into_iter().product::<u32>())
        })
        .sum()
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
        assert_eq!(467835, process(input));
    }
}
