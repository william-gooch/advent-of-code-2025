use anyhow::{Ok, anyhow};
use itertools::Itertools;

use super::{AOCChallenge, AOCResult};

fn max_single_digit(arr: &[u8]) -> Option<(usize, u8)> {
    let mut max = None;
    for (i, d) in arr.iter().copied().enumerate() {
        if d == 9 {
            return Some((i, d));
        }
        if let Some((m_i, m_d)) = max {
            if d > m_d {
                max = Some((i, d))
            }
        } else {
            max = Some((i, d))
        }
    }

    max
}

#[derive(Debug, Default)]
pub struct Challenge;

impl Challenge {
    fn largest_joltage_2(batteries: &[u8]) -> u64 {
        let (first_digit_loc, first_digit) =
            max_single_digit(&batteries[..(batteries.len() - 1)]).unwrap();
        let (second_digit_loc, second_digit) =
            max_single_digit(&batteries[(first_digit_loc + 1)..]).unwrap();

        (first_digit * 10 + second_digit).into()
    }

    fn largest_joltage(batteries: &[u8], to_turn_on: usize) -> u64 {
        let mut startpoint = 0;
        let mut string = 0;
        for i in (0..to_turn_on).rev() {
            println!(">> {startpoint}");
            let (digit_loc, digit) =
                max_single_digit(&batteries[startpoint..(batteries.len() - i)]).unwrap();

            startpoint += digit_loc + 1;
            string = (string * 10) + (digit as u64);
        }

        string
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let (batteries_1, batteries_2) = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).map(|d| d as u8).ok_or(()))
                    .collect::<Result<Vec<_>, ()>>()
                    .unwrap()
            })
            .tee();

        let largest_joltage_2: u64 = batteries_1
            .map(|batteries| Self::largest_joltage(batteries.as_slice(), 2))
            .sum();

        let largest_joltage_12: u64 = batteries_2
            .map(|batteries| Self::largest_joltage(batteries.as_slice(), 12))
            .sum();

        Ok(AOCResult {
            part_1: largest_joltage_2.to_string(),
            part_2: largest_joltage_12.to_string(),
        })
    }
}
