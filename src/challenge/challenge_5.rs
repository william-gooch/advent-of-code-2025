use fxhash::FxHashSet;
use itertools::Itertools;
use std::ops::RangeInclusive;

use anyhow::{Result, anyhow};

use super::{AOCChallenge, AOCResult};

#[derive(Default)]
struct RangeSet(FxHashSet<RangeInclusive<u64>>);

impl FromIterator<RangeInclusive<u64>> for RangeSet {
    fn from_iter<T: IntoIterator<Item = RangeInclusive<u64>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl RangeSet {
    fn contains(&self, item: &u64) -> bool {
        self.0.iter().any(|range| range.contains(item))
    }

    fn try_combine_ranges(
        first: &RangeInclusive<u64>,
        second: &RangeInclusive<u64>,
    ) -> Option<RangeInclusive<u64>> {
        let (first, second) = {
            if first.start() < second.start() {
                (first, second)
            } else {
                (second, first)
            }
        };

        if first.end() >= second.start() {
            let start = u64::min(*first.start(), *second.start());
            let end = u64::max(*first.end(), *second.end());
            Some(start..=end)
        } else {
            None
        }
    }

    fn coalesce_ranges(&mut self) {
        let mut sets_to_remove: FxHashSet<RangeInclusive<u64>> = Default::default();
        let mut sets_to_add: FxHashSet<RangeInclusive<u64>> = Default::default();

        loop {
            self.0
                .iter()
                .cloned()
                .array_combinations()
                .for_each(|[first, second]| {
                    // Don't try to coalesce ranges that are already marked for removal.
                    if sets_to_remove.contains(&first) || sets_to_remove.contains(&second) {
                        return;
                    }

                    if let Some(new_range) = Self::try_combine_ranges(&first, &second) {
                        sets_to_add.insert(new_range);
                        sets_to_remove.insert(first);
                        sets_to_remove.insert(second);
                    }
                });

            if sets_to_add.is_empty() {
                break;
            }

            self.0 = &(&self.0 - &sets_to_remove) | &sets_to_add;
            sets_to_remove.clear();
            sets_to_add.clear();
        }
    }

    fn size_of_ranges(&self) -> u64 {
        self.0
            .iter()
            .map(|range| (range.end() - range.start()) + 1)
            .sum()
    }
}

#[derive(Debug, Default)]
pub struct Challenge;

impl Challenge {}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let mut lines = input.lines();
        let mut ranges = lines
            .by_ref()
            .take_while(|line| line.len() > 0)
            .map(|line| -> Result<RangeInclusive<u64>> {
                let (start, end) = line
                    .split_once('-')
                    .ok_or(anyhow!("Invalid range notation"))?;

                Ok(u64::from_str_radix(start, 10)?..=u64::from_str_radix(end, 10)?)
            })
            .collect::<Result<RangeSet>>()
            .unwrap();

        ranges.coalesce_ranges();

        let items_in_ranges = lines
            .skip(1)
            .map(|line| Ok(u64::from_str_radix(line, 10)?))
            .collect::<Result<Vec<_>>>()
            .unwrap()
            .into_iter()
            .filter(|item| ranges.contains(&item))
            .count();

        Ok(AOCResult {
            part_1: items_in_ranges.to_string(),
            part_2: ranges.size_of_ranges().to_string(),
        })
    }
}
