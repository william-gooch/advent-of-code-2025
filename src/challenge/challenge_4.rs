use anyhow::{Result, anyhow};
use ndarray::{Array2, s};
use rayon::iter::{ParallelBridge, ParallelIterator};

use super::{AOCChallenge, AOCResult};

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Paper,
}

#[derive(Debug, Default)]
pub struct Challenge;

impl Challenge {
    fn get_accessible_rolls(arr: &Array2<Tile>) -> Vec<(usize, usize)> {
        let accessible_rolls = arr
            .indexed_iter()
            .par_bridge()
            .filter_map(|(idx, tile)| {
                if *tile == Tile::Empty {
                    None
                } else {
                    let r0 = usize::saturating_sub(idx.0, 1);
                    let c0 = usize::saturating_sub(idx.1, 1);
                    let r1 = usize::min(idx.0 + 1, arr.shape()[0] - 1);
                    let c1 = usize::min(idx.1 + 1, arr.shape()[1] - 1);

                    let window = arr.slice(s![r0..=r1, c0..=c1]);

                    (window.iter().filter(|i| **i == Tile::Paper).count() < 5).then_some(idx)
                }
            })
            .collect::<Vec<_>>();

        accessible_rolls
    }

    fn remove_rolls(arr: &mut Array2<Tile>) -> Option<usize> {
        let accessible_rolls = Self::get_accessible_rolls(arr);

        accessible_rolls
            .iter()
            .for_each(|idx| arr[*idx] = Tile::Empty);

        match accessible_rolls.len() {
            0 => None,
            x => Some(x),
        }
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();

        let tiles_flat = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|ch| match ch {
                    '@' => Tile::Paper,
                    _ => Tile::Empty,
                })
            })
            .collect::<Vec<_>>();

        let mut tiles = Array2::from_shape_vec((rows, cols), tiles_flat)
            .map_err(|_| anyhow!("Couldn't construct array"))?;

        let num_accessible_rolls = Self::get_accessible_rolls(&tiles).len();

        let num_removed_rolls: usize = (0..).map_while(|_| Self::remove_rolls(&mut tiles)).sum();

        Ok(AOCResult {
            part_1: num_accessible_rolls.to_string(),
            part_2: num_removed_rolls.to_string(),
        })
    }
}
