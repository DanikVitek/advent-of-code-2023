use crate::game::{hand, Game};

pub fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            Game::<hand::Normalized>::parse(line)
                .expect("invalid input")
                .1
        })
        .flat_map(|game| {
            game.hands
                .into_iter()
                .map(|h| h.0)
                .reduce(|a, b| hand::Normalized {
                    red_amount: a.red_amount.max(b.red_amount),
                    green_amount: a.green_amount.max(b.green_amount),
                    blue_amount: a.blue_amount.max(b.blue_amount),
                })
                .map(
                    |hand::Normalized {
                         red_amount,
                         green_amount,
                         blue_amount,
                     }| red_amount * green_amount * blue_amount,
                )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(input));
    }
}
