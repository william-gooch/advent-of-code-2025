use std::{cell::LazyCell, ops::BitOr};

use anyhow::anyhow;
use fxhash::FxHashSet;
use itertools::Itertools;
use ndarray::{Zip, prelude::*};

use super::{AOCChallenge, AOCResult};

#[derive(Debug, Default)]
pub struct Challenge;

fn disp_shape<'a>(shape: impl AsArray<'a, bool, Ix2>) -> String {
    shape
        .into()
        .axis_iter(Axis(0))
        .map(|ax| {
            ax.iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        })
        .join("\n")
}

struct Present<'a> {
    shape: ArrayView2<'a, bool>,
    variants: Box<[ArrayView2<'a, bool>]>,
    num_cells: usize,
}

impl<'a> Present<'a> {
    fn new(shape: &'a Array2<bool>) -> Self {
        let variants = [s![.., ..], s![..;-1, ..], s![..;-1, ..;-1], s![.., ..;-1]]
            .iter()
            .map(|slice| shape.slice(slice))
            .flat_map(|var| [var, var.reversed_axes()])
            .unique()
            .collect::<Box<[_]>>();

        let num_cells = shape.iter().filter(|b| **b).count();

        Self {
            shape: shape.into(),
            variants,
            num_cells,
        }
    }
}

struct Region<'a> {
    width: usize,
    height: usize,
    requirements: Vec<usize>,

    presents: &'a Vec<Present<'a>>,
    zeros: Array2<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    map: Array2<bool>,
    remaining_presents: Vec<usize>,
    available_cells: usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}\n", disp_shape(&self.map))?;
        write!(f, "To Place: {:?}", self.remaining_presents)?;

        Ok(())
    }
}

impl State {
    fn new(map: Array2<bool>, present_count: Vec<usize>) -> Self {
        let available_cells = map.iter().filter(|b| !**b).count();

        Self {
            map,
            remaining_presents: present_count,
            available_cells,
        }
    }
}

impl<'a> Region<'a> {
    fn new(
        width: usize,
        height: usize,
        requirements: Vec<usize>,
        presents: &'a Vec<Present<'a>>,
    ) -> Self {
        Self {
            width,
            height,
            requirements,
            presents,
            zeros: Array2::from_shape_simple_fn((height, width), || false),
        }
    }

    fn init_state(&self) -> State {
        let map = self.zeros.clone();
        let present_count = self.requirements.clone();

        State::new(map, present_count)
    }

    fn easy_is_unsolvable(&self, state: &State) -> bool {
        let remaining_cells = state
            .remaining_presents
            .iter()
            .enumerate()
            .map(|(idx, remaining)| remaining * self.presents[idx].num_cells)
            .sum();

        state.available_cells < remaining_cells
    }

    fn try_solve(&self, state: State, cache: &mut FxHashSet<State>) -> bool {
        // println!("Searched: {}", cache.len());
        // println!("Checking: {state}");
        cache.insert(state.clone());

        if state.remaining_presents.iter().all(|p| *p == 0) {
            println!("Found: {state}");
            return true;
        }

        if self.easy_is_unsolvable(&state) {
            return false;
        }

        let (state, nexts) = self.valid_options(state);
        let nexts = nexts.filter(|st| !cache.contains(&st)).collect::<Vec<_>>();
        cache.insert(state.clone());
        // println!("Pruned: {state}");

        if self.easy_is_unsolvable(&state) {
            return false;
        }

        nexts.into_iter().any(|st| self.try_solve(st, cache))
    }

    fn place_shape(
        &self,
        state: &State,
        present_idx: usize,
        shape: ArrayView2<bool>,
        position: Ix2,
    ) -> Option<State> {
        let mut new_state = state.clone();

        if new_state.remaining_presents[present_idx] == 0 {
            return None;
        }
        new_state.remaining_presents[present_idx] -= 1;

        let mut view = new_state.map.slice_mut(s![
            position[0] - 1..=position[0] + 1,
            position[1] - 1..=position[1] + 1
        ]);

        if (&view & &shape).iter().all(|b| !b) {
            view |= &shape;
            Some(new_state)
        } else {
            None
        }
    }

    fn position_valid_options(&self, state: &State, position: Ix2) -> impl Iterator<Item = State> {
        self.presents
            .iter()
            .enumerate()
            .filter(|&(present_idx, _)| state.remaining_presents[present_idx] > 0)
            .flat_map(|(present_idx, present)| {
                present.variants.iter().map(move |p| (present_idx, p))
            })
            .filter_map(|(present_idx, v)| self.place_shape(&state, present_idx, *v, position))
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn valid_options(&self, mut state: State) -> (State, impl Iterator<Item = State>) {
        let mut invalid_position_map = Array2::from_shape_simple_fn(state.map.raw_dim(), || false);
        let positions = invalid_position_map.indexed_iter_mut();

        let position_states = positions
            .filter(|&((row, col), _)| {
                !(row == 0 || col == 0 || row == self.height - 1 || col == self.width - 1)
            })
            .map(|((row, col), cell)| (Ix2(row, col), cell))
            .map(|(position, cell)| {
                (
                    position,
                    cell,
                    Self::position_valid_options(&self, &state, position).collect::<Vec<_>>(),
                )
            })
            .flat_map(|(position, cell, options)| {
                if options.is_empty() {
                    *cell = true;
                }
                options
            })
            .collect::<Vec<_>>();

        let unreachable_positions = !position_states.iter().map(|st| &st.map).fold(
            Array2::from_shape_simple_fn((self.height, self.width), || false),
            BitOr::bitor,
        );

        // Fill in cell locations that can't be filled in with a present.
        let state = State::new(state.map | unreachable_positions, state.remaining_presents);

        let pruned_position_states = position_states
            .into_iter()
            .map(|st| State::new(&st.map | &state.map, st.remaining_presents))
            .collect::<Vec<_>>();

        (state, pruned_position_states.into_iter())
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let mut lines = input.lines();

        let present_arrs = lines
            .by_ref()
            .take(30)
            .chunks(5)
            .into_iter()
            .map(|chunk| {
                let tiles = chunk
                    .skip(1)
                    .take(3)
                    .flat_map(|l| {
                        l.chars().map(|ch| match ch {
                            '#' => true,
                            '.' => false,
                            _ => panic!(),
                        })
                    })
                    .collect::<Vec<_>>();
                Array2::from_shape_vec((3, 3), tiles).unwrap()
            })
            .collect::<Vec<_>>();

        let presents = present_arrs
            .iter()
            .map(|shape| Present::new(shape))
            .collect::<Vec<_>>();

        presents.iter().for_each(|p| {
            p.variants
                .iter()
                .for_each(|v| println!("{}", disp_shape(v)));
            println!("---");
        });

        let regions = lines
            .map(|line| line.split_once(": ").unwrap())
            .map(|(size_str, nums_str)| {
                let (w_str, h_str) = size_str.split_once("x").unwrap();
                let width = w_str.parse::<usize>().unwrap();
                let height = h_str.parse::<usize>().unwrap();

                let requirements = nums_str
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                Region::new(width, height, requirements, &presents)
            })
            .collect::<Vec<_>>();

        // let solvable_regions = regions
        //     .iter()
        //     .filter(|region| {
        //         let mut cache = Default::default();
        //         region.try_solve(region.init_state(), &mut cache)
        //     })
        //     .count();

        let solvable_regions = regions
            .iter()
            .filter(|region| {
                region
                    .requirements
                    .iter()
                    .enumerate()
                    .map(|(i, p)| p * presents[i].num_cells)
                    .sum::<usize>()
                    <= (region.width * region.height)
            })
            .count();

        Ok(AOCResult {
            part_1: solvable_regions.to_string(),
            part_2: 0.to_string(),
        })
    }
}
