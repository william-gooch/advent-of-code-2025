use std::{
    cell::LazyCell,
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
    str::FromStr,
};

use anyhow::{Result, anyhow};
use fxhash::FxHashSet;
use good_lp::*;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector, LU, SVD, zero};
use rayon::prelude::*;
use regex::Regex;

use super::{AOCChallenge, AOCResult};

const EPS: f64 = 0.00001;
fn is_whole(f: f64) -> bool {
    (f - f.round()).abs() < EPS
}

#[derive(Debug, Default)]
pub struct Challenge;

#[derive(Debug)]
struct Machine {
    light_requirements: Lights,
    buttons: Vec<Box<[usize]>>,
    joltage_requirements: Box<[u64]>,
}

#[derive(Debug)]
struct HeapElement {
    state: Box<[u64]>,
    presses: u64,
    distance_heuristic: u64,
}

impl HeapElement {
    fn try_new(state: Box<[u64]>, presses: u64, target_state: &[u64]) -> Option<Self> {
        let distance_heuristic = target_state
            .iter()
            .zip(state.iter())
            .map(|(t, s)| t.checked_sub(*s))
            .sum::<Option<u64>>()?;

        Some(Self {
            state,
            presses,
            distance_heuristic,
        })
    }

    fn try_press(&self, button: &[usize], target_state: &[u64], n_times: u64) -> Option<Self> {
        let mut new_state = self.state.clone();
        button.iter().for_each(|bit| new_state[*bit] += n_times);

        Self::try_new(new_state, self.presses + n_times, target_state)
    }
}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for HeapElement {}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.presses + self.distance_heuristic)
            .cmp(&(other.presses + other.distance_heuristic))
            .reverse()
    }
}

type Lights = u64;

impl Machine {
    fn optimise_lights(&self) -> u64 {
        let mut queue: VecDeque<(Lights, u64)> = Default::default();
        queue.push_back((0, 0));
        let mut visited: FxHashSet<Lights> = Default::default();
        let button_masks = self
            .buttons
            .iter()
            .map(|bits| {
                let mut l = 0;
                bits.iter()
                    .for_each(|b| l |= 1 << (self.joltage_requirements.len() - b - 1));
                l
            })
            .collect::<Vec<_>>();

        while let Some((curr, presses)) = queue.pop_front() {
            visited.insert(curr.clone());

            if curr == self.light_requirements {
                return presses;
            }

            button_masks
                .iter()
                .map(|button| curr.clone() ^ button)
                .filter(|next| !visited.contains(next))
                .for_each(|next| queue.push_back((next, presses + 1)));
        }

        0
    }

    // fn optimise_joltages(&self) -> u64 {
    //     let mut queue: BinaryHeap<HeapElement> = Default::default();
    //     queue.push(
    //         HeapElement::try_new(
    //             vec![0; self.joltage_requirements.len()].into_boxed_slice(),
    //             0,
    //             &self.joltage_requirements,
    //         )
    //         .expect("Couldn't add initial state??"),
    //     );

    //     let mut visited: FxHashSet<Box<[u64]>> = Default::default();

    //     while let Some(curr) = queue.pop() {
    //         println!("{curr:?}");

    //         if curr.state == self.joltage_requirements {
    //             return curr.presses;
    //         }

    //         self.buttons
    //             .iter()
    //             .flat_map(|button| {
    //                 (1..).map_while(|n_times| {
    //                     curr.try_press(button, &self.joltage_requirements, n_times)
    //                 })
    //             })
    //             .filter(|next| !visited.contains(&next.state))
    //             .for_each(|next| queue.push(next));

    //         visited.insert(curr.state);
    //     }

    //     0
    // }

    fn optimise_joltages(&self) -> u64 {
        let mut problem = ProblemVariables::new();
        let button_vars: Vec<Variable> =
            problem.add_all((0..self.buttons.len()).map(|_| variable().min(0).integer()));

        let objective: Expression = button_vars.iter().sum();

        let joltage_to_buttons =
            self.joltage_requirements
                .iter()
                .copied()
                .enumerate()
                .map(|(i, req)| {
                    let req = req as u32;
                    let relevant_buttons = self
                        .buttons
                        .iter()
                        .enumerate()
                        .filter_map(|(button, joltages)| joltages.contains(&i).then_some(button))
                        .map(|button| button_vars[button])
                        .sum::<Expression>();

                    constraint!(relevant_buttons == req)
                });

        let solution = problem
            .minimise(&objective)
            .using(default_solver)
            .with_all(joltage_to_buttons)
            .solve()
            .unwrap();

        solution.eval(&objective) as u64
    }
}

const PARSE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"\[(?<lights>[.#]+)\] (?<buttons>(?:\([\d,]+\) ?)+) \{(?<joltage>(?:\d+,?)+)\}")
        .unwrap()
});

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PARSE_REGEX
            .captures(s)
            .ok_or(anyhow!("Couldn't parse string"))?;

        let lights_str = captures
            .name("lights")
            .ok_or(anyhow!("Couldn't parse lights"))?
            .as_str();
        let buttons_str = captures
            .name("buttons")
            .ok_or(anyhow!("Couldn't parse buttons"))?
            .as_str();
        let joltage_str = captures
            .name("joltage")
            .ok_or(anyhow!("Couldn't parse joltage"))?
            .as_str();

        println!("[{lights_str}] {buttons_str} {{{joltage_str}}}");

        let (lights, num_lights) = lights_str
            .chars()
            .map(|ch| ch == '#')
            .fold((0, 0), |(s, n), b| ((s << 1) | (b as u64), n + 1));
        let buttons = buttons_str
            .split_whitespace()
            .map(|button_str| {
                button_str
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .map(|num| Ok(str::parse::<usize>(num)?))
                    .collect::<Result<Box<[_]>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        let joltages = joltage_str
            .split(",")
            .map(|num| Ok(str::parse::<u64>(num)?))
            .collect::<Result<Box<[u64]>>>()?;

        Ok(Self {
            light_requirements: lights,
            buttons,
            joltage_requirements: joltages,
        })
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let machines = input
            .lines()
            .map(|line| Machine::from_str(line))
            .collect::<Result<Vec<_>>>()?;

        let lights_presses: u64 = machines.par_iter().map(Machine::optimise_lights).sum();
        let joltage_presses: u64 = machines.iter().map(Machine::optimise_joltages).sum();

        Ok(AOCResult {
            part_1: lights_presses.to_string(),
            part_2: joltage_presses.to_string(),
        })
    }
}
