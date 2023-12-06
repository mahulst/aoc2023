use nom::{
    bytes::complete::{self, tag},
    character::{
        self,
        complete::{alpha1, char, digit1, line_ending},
    },
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let answer_1 = part_1(&[
        Race {
            time: 53,
            distance: 275,
        },
        Race {
            time: 71,
            distance: 1181,
        },
        Race {
            time: 78,
            distance: 1218,
        },
        Race {
            time: 0,
            distance: 0,
        },
    ]);
    let answer_2 = part_1(&[Race {
        time: 53717880,
        distance: 275118112151524,
    }]);

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}
struct Race {
    time: usize,
    distance: usize,
}

fn do_race(race: &Race) -> usize {
    (0..race.time)
        .into_par_iter()
        .map(|time_held| {
            let time_travelled = race.time - time_held;
            let distance = time_travelled * time_held;

            distance
        })
        .filter(|travelled| travelled > &race.distance)
        .count()
}
fn part_1(input: &[Race]) -> usize {
    input.iter().map(do_race).product()
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_day_1() {
        let input = [
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ];

        let result = part_1(&input);
        assert_eq!(result, 288);
    }
    #[test]
    fn test_day_2() {
        let input = [Race {
            time: 71530,
            distance: 940200,
        }];

        let result = part_1(&input);
        assert_eq!(result, 71503);
    }
}
