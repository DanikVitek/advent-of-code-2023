use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};
use rayon::prelude::*;

pub fn process(input: &str) -> u32 {
    let (_, (seeds, mappings)) = pair(
        delimited(tag("seeds: "), Seeds::parse, pair(line_ending, line_ending)),
        Mappings::parse,
    )(input)
    .expect("invalid input");

    seeds
        .into_iter()
        .map(|seed| mappings.apply(seed))
        .min()
        .expect("invalid input")
}

struct Mappings {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Mappings {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                delimited(
                    pair(tag("seed-to-soil map:"), line_ending),
                    Map::parse,
                    pair(line_ending, line_ending),
                ),
                delimited(
                    pair(tag("soil-to-fertilizer map:"), line_ending),
                    Map::parse,
                    pair(line_ending, line_ending),
                ),
                delimited(
                    pair(tag("fertilizer-to-water map:"), line_ending),
                    Map::parse,
                    pair(line_ending, line_ending),
                ),
                delimited(
                    pair(tag("water-to-light map:"), line_ending),
                    Map::parse,
                    pair(line_ending, line_ending),
                ),
                delimited(
                    pair(tag("light-to-temperature map:"), line_ending),
                    Map::parse,
                    pair(line_ending, line_ending),
                ),
                delimited(
                    pair(tag("temperature-to-humidity map:"), line_ending),
                    Map::parse,
                    pair(line_ending, line_ending),
                ),
                preceded(
                    pair(tag("humidity-to-location map:"), line_ending),
                    Map::parse,
                ),
            )),
            |(
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            )| Self {
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            },
        )(input)
    }

    fn apply(&self, seed: u32) -> u32 {
        let soil = self.seed_to_soil.apply(seed);
        let fertilizer = self.soil_to_fertilizer.apply(soil);
        let water = self.fertilizer_to_water.apply(fertilizer);
        let light = self.water_to_light.apply(water);
        let temperature = self.light_to_temperature.apply(light);
        let humidity = self.temperature_to_humidity.apply(temperature);
        return self.humidity_to_location.apply(humidity);
    }
}

#[derive(Debug, Clone, Copy)]
struct SeedsRange {
    start: u32,
    len: u32,
}

impl SeedsRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        let start = map_res(digit1, |input: &str| input.parse::<u32>());
        let len = map_res(digit1, |input: &str| input.parse::<u32>());
        map(separated_pair(start, char(' '), len), |(start, len)| Self {
            start,
            len,
        })(input)
    }
}

impl IntoIterator for SeedsRange {
    type Item = u32;
    type IntoIter = std::ops::Range<u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..self.start + self.len
    }
}

impl IntoParallelIterator for SeedsRange {
    type Iter = rayon::range::Iter<u32>;
    type Item = u32;

    fn into_par_iter(self) -> Self::Iter {
        (self.start..self.start + self.len).into_par_iter()
    }
}

struct Seeds {
    ranges: Vec<SeedsRange>,
}

impl Seeds {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(char(' '), SeedsRange::parse), |ranges| {
            Self { ranges }
        })(input)
    }
}

impl IntoIterator for Seeds {
    type Item = u32;
    type IntoIter = std::iter::Flatten<std::vec::IntoIter<SeedsRange>>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter().flatten().into_iter()
    }
}

impl IntoParallelIterator for Seeds {
    type Iter = rayon::iter::Flatten<rayon::vec::IntoIter<SeedsRange>>;
    type Item = u32;

    fn into_par_iter(self) -> Self::Iter {
        self.ranges.into_par_iter().flatten()
    }
}

#[derive(Debug, Clone, Copy)]
struct RangeMap {
    destination_start: u32,
    source_start: u32,
    len: u32,
}

impl RangeMap {
    const fn apply(self, source: u32) -> Option<u32> {
        if self.source_start <= source && source - self.source_start < self.len {
            Some(self.destination_start + (source - self.source_start))
        } else {
            None
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let destination_start = map_res(digit1, |input: &str| input.parse::<u32>());
        let source_start = map_res(digit1, |input: &str| input.parse::<u32>());
        let len = map_res(digit1, |input: &str| input.parse::<u32>());
        map(
            tuple((destination_start, char(' '), source_start, char(' '), len)),
            |(destination_start, _, source_start, _, len)| Self {
                destination_start,
                source_start,
                len,
            },
        )(input)
    }
}

struct Map {
    ranges: Box<[RangeMap]>,
}

impl Map {
    fn apply(&self, source: u32) -> u32 {
        self.ranges
            .binary_search_by(|range| {
                if range.source_start > source {
                    Ordering::Greater
                } else if source - range.source_start >= range.len {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .map(|range| {
                unsafe { self.ranges.get_unchecked(range) }
                    .apply(source)
                    .unwrap()
            })
            .unwrap_or(source)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list0(line_ending, RangeMap::parse),
            |mut ranges| {
                ranges.sort_unstable_by_key(|range| range.source_start);
                Self {
                    ranges: ranges.into_boxed_slice(),
                }
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(46, process(input));
    }
}
