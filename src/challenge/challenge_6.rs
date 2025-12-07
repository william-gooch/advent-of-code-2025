use anyhow::{Result, anyhow};
use itertools::Itertools;
use ndarray::{Array2, Axis};
use rayon::{iter::ParallelIterator, str::ParallelString};

use super::{AOCChallenge, AOCResult};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, Default)]
pub struct Challenge;

impl Challenge {
    fn parse_table_p1<'a>(
        lines: impl Iterator<Item = &'a str>,
        ops: impl Iterator<Item = Op>,
    ) -> Result<u64> {
        let table = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|num| Ok(u64::from_str_radix(num, 10)?))
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        let numbers = crate::utils::ragged_to_arr(table)?;

        let grand_total: u64 = numbers
            .axis_iter(ndarray::Axis(1))
            .zip(ops)
            .map(|(ax, op)| {
                println!("{ax:?} {op:?}");
                match op {
                    Op::Add => ax.sum(),
                    Op::Mul => ax.product(),
                }
            })
            .sum();

        Ok(grand_total)
    }

    fn parse_table_p2<'a>(
        lines: impl Iterator<Item = &'a str>,
        ops: impl Iterator<Item = Op>,
    ) -> Result<u64> {
        let chars_ragged = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let chars_arr = crate::utils::ragged_to_arr(chars_ragged)?;

        let groups = chars_arr
            .t()
            .axis_iter(Axis(0))
            .map(|ax| u64::from_str_radix(ax.iter().collect::<String>().trim(), 10))
            .collect::<Vec<_>>()
            .into_iter()
            .chunk_by(|num| num.is_ok());

        let table = groups
            .into_iter()
            .filter_map(|(is_ok, chunk)| is_ok.then(|| chunk.flatten().collect::<Vec<_>>()));

        let grand_total: u64 = table
            .zip(ops)
            .map(|(ax, op)| -> u64 {
                println!("{ax:?} {op:?}");
                match op {
                    Op::Add => ax.iter().sum(),
                    Op::Mul => ax.iter().product(),
                }
            })
            .sum();

        Ok(grand_total)
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> Result<AOCResult> {
        let mut lines = input.lines();

        let ops = lines
            .by_ref()
            .next_back()
            .ok_or(anyhow!("No last line"))?
            .split_whitespace()
            .map(|op| match op {
                "+" => Ok(Op::Add),
                "*" => Ok(Op::Mul),
                _ => Err(anyhow!("Invalid operation")),
            })
            .collect::<Result<Vec<_>>>()?;

        let (lines_1, lines_2) = lines.tee();

        let grand_total_1 = Self::parse_table_p1(lines_1, ops.iter().copied())?;
        let grand_total_2 = Self::parse_table_p2(lines_2, ops.into_iter())?;

        Ok(AOCResult {
            part_1: grand_total_1.to_string(),
            part_2: grand_total_2.to_string(),
        })
    }
}
