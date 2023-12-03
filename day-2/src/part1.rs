use crate::game::{hand, Game, Hand};

pub fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            Game::<hand::Normalized>::parse(line)
                .expect("invalid input")
                .1
        })
        .filter(is_valid_game)
        .map(|game| game.id)
        .sum()
}

fn is_valid_game(game: &Game<hand::Normalized>) -> bool {
    const RED_CUBES: usize = 12;
    const GREEN_CUBES: usize = 13;
    const BLUE_CUBES: usize = 14;

    game.hands.iter().all(|Hand(hand)| {
        hand.red_amount <= RED_CUBES
            && hand.green_amount <= GREEN_CUBES
            && hand.blue_amount <= BLUE_CUBES
    })
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
        assert_eq!(8, process(input));
    }
}
