use nom::{
    bytes::complete::{tag, take_until},
    character::{
        self,
        complete::{line_ending, space1},
    },
    multi::separated_list1,
    IResult,
};
use rayon::prelude::*;
use std::{collections::HashMap, ops::Range};

fn main() {
    let answer_1 = part_1(
        include_str!("./input.txt"),
        vec![
            768975, 36881621, 56868281, 55386784, 1828225758, 1084205557, 2956956868, 127170752,
            1117192172, 332560644, 357791695, 129980646, 819363529, 9145257, 993170544, 70644734,
            3213715789, 312116873, 3107544690, 59359615,
        ],
    );
    let answer_2 = part_2(
        include_str!("./input.txt"),
        vec![
            768975..768975 + 36881621,
            56868281..56868281 + 55386784,
            1828225758..1828225758 + 1084205557,
            2956956868..2956956868 + 127170752,
            1117192172..1117192172 + 332560644,
            357791695..357791695 + 129980646,
            819363529..819363529 + 9145257,
            993170544..993170544 + 70644734,
            3213715789..3213715789 + 312116873,
            3107544690..3107544690 + 59359615,
        ],
    );

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}
#[derive(Debug)]
struct Transformer {
    base: usize,
    source: Range<usize>,
}

#[derive(Debug)]
struct Mapper {
    transformers: Vec<Transformer>,
}

fn transformer(input: &str) -> IResult<&str, Transformer> {
    let (input, transformer) = separated_list1(space1, character::complete::u32)(input)?;
    let base = transformer[0] as usize;
    let start = transformer[1] as usize;
    let end = transformer[2] as usize + start as usize;
    Ok((
        input,
        Transformer {
            base,
            source: Range { start, end },
        },
    ))
}
fn mapper(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, mapper) = separated_list1(line_ending, transformer)(input)?;

    Ok((
        input,
        Mapper {
            transformers: mapper,
        },
    ))
}

fn mappers(input: &str) -> IResult<&str, Vec<Mapper>> {
    let (input, g) = separated_list1(tag("\n\n"), mapper)(input)?;
    Ok((input, g))
}

fn part_1(input: &str, seeds: Vec<usize>) -> usize {
    println!("running total of {}", seeds.len());
    let (_, mappers) = mappers(input).unwrap();
    seeds
        .par_iter()
        .map(|seed| {
            mappers.iter().fold(*seed, |acc, mapper| {
                let mapped = mapper.transformers.iter().find(|t| t.source.contains(&acc));
                let value = match (mapped) {
                    Some(trans) => {
                        let index = acc - trans.source.start;
                        index + trans.base
                    }
                    None => acc,
                };

                value
            })
        })
        .min()
        .unwrap()
}
fn part_2(input: &str, seeds: Vec<Range<usize>>) -> usize {
    let nums: Vec<usize> = seeds.into_iter().flat_map(|range| range).collect();

    part_1(input, nums)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn parse() {
        let input = "seed-to-soil map:
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
        let result = mappers(input).unwrap();
    }

    #[test]
    fn test_day_1() {
        let input = "seed-to-soil map:
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
        let result = part_1(input, vec![79, 14, 55, 13]);
        assert_eq!(result, 35);
    }
    #[test]
    fn test_day_2() {
        let input = "seed-to-soil map:
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
        let result = part_2(input, vec![79..79 + 14, 55..55 + 13]);
        assert_eq!(result, 46);
    }
}
