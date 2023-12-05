use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Game<Normalization> {
    pub id: usize,
    pub hands: Vec<Hand<Normalization>>,
}

impl Game<hand::Unnormalized> {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let id = delimited(tag("Game "), usize, tag(": "));
        let hands = separated_list1(tag("; "), Hand::parse);
        map(pair(id, hands), |(id, hands)| Self { id, hands })(input)
    }

    pub fn normalized(self) -> Game<hand::Normalized> {
        Game {
            id: self.id,
            hands: self.hands.into_iter().map(Hand::normalized).collect(),
        }
    }
}

impl Game<hand::Normalized> {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        Game::<hand::Unnormalized>::parse(input).map(|(input, game)| (input, game.normalized()))
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Hand<Normalization>(pub Normalization);

pub mod hand {
    use super::Cubes;

    #[derive(Debug)]
    #[cfg_attr(test, derive(PartialEq, Eq))]
    #[repr(transparent)]
    pub struct Unnormalized(pub Vec<Cubes>);

    #[derive(Debug, Default)]
    #[cfg_attr(test, derive(PartialEq, Eq))]
    pub struct Normalized {
        pub red_amount: usize,
        pub green_amount: usize,
        pub blue_amount: usize,
    }
}

impl Hand<hand::Unnormalized> {
    #[inline(always)]
    pub fn new_unnormalized(cubes: Vec<Cubes>) -> Self {
        Self(hand::Unnormalized(cubes))
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tag(", "), Cubes::parse),
            Self::new_unnormalized,
        )(input)
    }

    pub fn normalized(self) -> Hand<hand::Normalized> {
        let values: Vec<Cubes> = self.0 .0;
        Hand(values.into_iter().fold(
            hand::Normalized::default(),
            |mut acc, Cubes { amount, color }| {
                match color {
                    Color::Red => acc.red_amount += amount,
                    Color::Green => acc.green_amount += amount,
                    Color::Blue => acc.blue_amount += amount,
                };
                acc
            },
        ))
    }
}

impl Hand<hand::Normalized> {
    #[inline(always)]
    pub fn new_normalized(red: usize, green: usize, blue: usize) -> Self {
        Self(hand::Normalized {
            red_amount: red,
            green_amount: green,
            blue_amount: blue,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Cubes {
    pub amount: usize,
    pub color: Color,
}

impl Cubes {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(usize, char(' '), Color::parse),
            |(amount, color)| Self { amount, color },
        )(input)
    }
}

fn usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, <str>::parse)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Red, tag("red")),
            value(Self::Green, tag("green")),
            value(Self::Blue, tag("blue")),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game {
        id: 1,
        hands: vec![
            Hand::new_unnormalized(vec![
                Cubes { amount: 3, color: Color::Blue },
                Cubes { amount: 4, color: Color::Red },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 1, color: Color::Red },
                Cubes { amount: 2, color: Color::Green },
                Cubes { amount: 6, color: Color::Blue },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 2, color: Color::Green },
            ]),
        ],
    })]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Game {
        id: 2,
        hands: vec![
            Hand::new_unnormalized(vec![
                Cubes { amount: 1, color: Color::Blue },
                Cubes { amount: 2, color: Color::Green },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 3, color: Color::Green },
                Cubes { amount: 4, color: Color::Blue },
                Cubes { amount: 1, color: Color::Red },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 1, color: Color::Green },
                Cubes { amount: 1, color: Color::Blue },
            ]),
        ],
    })]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Game {
        id: 3,
        hands: vec![
            Hand::new_unnormalized(vec![
                Cubes { amount: 8, color: Color::Green },
                Cubes { amount: 6, color: Color::Blue },
                Cubes { amount: 20, color: Color::Red },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 5, color: Color::Blue },
                Cubes { amount: 4, color: Color::Red },
                Cubes { amount: 13, color: Color::Green },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 5, color: Color::Green },
                Cubes { amount: 1, color: Color::Red },
            ]),
        ],
    })]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Game {
        id: 4,
        hands: vec![
            Hand::new_unnormalized(vec![
                Cubes { amount: 1, color: Color::Green },
                Cubes { amount: 3, color: Color::Red },
                Cubes { amount: 6, color: Color::Blue },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 3, color: Color::Green },
                Cubes { amount: 6, color: Color::Red },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 3, color: Color::Green },
                Cubes { amount: 15, color: Color::Blue },
                Cubes { amount: 14, color: Color::Red },
            ]),
        ],
    })]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Game {
        id: 5,
        hands: vec![
            Hand::new_unnormalized(vec![
                Cubes { amount: 6, color: Color::Red },
                Cubes { amount: 1, color: Color::Blue },
                Cubes { amount: 3, color: Color::Green },
            ]),
            Hand::new_unnormalized(vec![
                Cubes { amount: 2, color: Color::Blue },
                Cubes { amount: 1, color: Color::Red },
                Cubes { amount: 2, color: Color::Green },
            ]),
        ],
    })]
    fn test_parse_game(#[case] input: &str, #[case] expected: Game<hand::Unnormalized>) {
        assert_eq!(
            expected,
            Game::<hand::Unnormalized>::parse(input).unwrap().1
        );
    }

    #[rstest]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 3, color: Color::Blue },
            Cubes { amount: 4, color: Color::Red },
        ]),
        Hand::new_normalized(4, 0, 3),)]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 1, color: Color::Red },
            Cubes { amount: 2, color: Color::Green },
            Cubes { amount: 6, color: Color::Blue },
        ]),
        Hand::new_normalized(1, 2, 6),
    )]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 2, color: Color::Green },
        ]),
        Hand::new_normalized(0, 2, 0),
    )]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 1, color: Color::Blue },
            Cubes { amount: 2, color: Color::Green },
        ]),
        Hand::new_normalized(0, 2, 1),
    )]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 3, color: Color::Green },
            Cubes { amount: 4, color: Color::Blue },
            Cubes { amount: 1, color: Color::Red },
        ]),
        Hand::new_normalized(1, 3, 4),
    )]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 1, color: Color::Green },
            Cubes { amount: 1, color: Color::Blue },
        ]),
        Hand::new_normalized(0, 1, 1),
    )]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 8, color: Color::Green },
            Cubes { amount: 6, color: Color::Blue },
            Cubes { amount: 20, color: Color::Red },
        ]),
        Hand::new_normalized(20, 8, 6),
    )]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 5, color: Color::Blue },
            Cubes { amount: 4, color: Color::Red },
            Cubes { amount: 13, color: Color::Green },
        ]),
        Hand::new_normalized(4, 13, 5),
    )]
    #[case(
        Hand::new_unnormalized(vec![
            Cubes { amount: 5, color: Color::Green },
            Cubes { amount: 1, color: Color::Red },
        ]),
        Hand::new_normalized(1, 5, 0),
    )]
    fn test_hand_normalized(
        #[case] input: Hand<hand::Unnormalized>,
        #[case] expected: Hand<hand::Normalized>,
    ) {
        assert_eq!(expected, input.normalized());
    }
}
