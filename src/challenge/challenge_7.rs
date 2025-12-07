use std::collections::VecDeque;

use anyhow::{Result, anyhow};
use fxhash::FxHashMap;
use itertools::Itertools;
use ndarray::{Array2, Axis, Ix2, s};

use crate::utils::ragged_to_arr;

use super::{AOCChallenge, AOCResult};

#[derive(Debug, Default)]
pub struct Challenge;

impl Challenge {
    fn trace_beam_split(map: &Array2<char>) -> u64 {
        let mut map = map.clone();
        let mut rows = map.axis_iter_mut(Axis(0));

        let mut source = rows.next().unwrap();

        let mut splits = 0;
        while let Some(mut dest) = rows.next() {
            for i in 0..source.len() {
                match dest[i] {
                    '.' => {
                        if source[i] == 'S' || source[i] == '|' {
                            dest[i] = '|'
                        }
                    }
                    '^' => {
                        if source[i] == '|' {
                            dest[i - 1] = '|';
                            dest[i + 1] = '|';
                            splits += 1;
                        }
                    }
                    _ => (),
                }
            }

            source = dest;
        }

        println!("{}", map);

        splits
    }

    fn trace_beam_choose(map: &Array2<char>) -> u64 {
        let mut choices_map: Array2<Option<u64>> =
            Array2::from_shape_fn(map.raw_dim(), |(row, col)| {
                if row == map.shape()[0] - 1 {
                    Some(1)
                } else {
                    None
                }
            });

        for row in (0..(choices_map.shape()[0] - 1)).rev() {
            for col in 0..choices_map.shape()[1] {
                match map[Ix2(row + 1, col)] {
                    '.' => choices_map[Ix2(row, col)] = choices_map[Ix2(row + 1, col)],
                    '^' => {
                        choices_map[Ix2(row, col)] = Some(
                            choices_map[Ix2(row + 1, col - 1)].unwrap()
                                + choices_map[Ix2(row + 1, col + 1)].unwrap(),
                        )
                    }
                    _ => (),
                }
            }
        }

        println!("{choices_map:?}");

        choices_map[Ix2(0, choices_map.shape()[1] / 2)].unwrap()
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> Result<AOCResult> {
        let chars_ragged = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let map = ragged_to_arr(chars_ragged)?;

        println!("{map:?}");

        let splits = Self::trace_beam_split(&map);
        let choices = Self::trace_beam_choose(&map);

        Ok(AOCResult {
            part_1: splits.to_string(),
            part_2: choices.to_string(),
        })
    }
}
