use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;

use super::{AOCChallenge, AOCResult};

const fn decimal_rsh(num: u64, digits: u32) -> u64 {
    let pow10 = 10u64.pow(digits);
    num / pow10
}

const fn decimal_lsh(num: u64, digits: u32) -> u64 {
    let pow10 = 10u64.pow(digits);
    num * pow10
}

fn decimal_substr(num: u64, start: u32, end: u32) -> u64 {
    let num_digits = num.ilog10() + 1;
    let lower = decimal_rsh(num, num_digits - end);
    // println!("{lower}");
    let upper = decimal_lsh(decimal_rsh(lower, end - start), end - start);
    // println!("{upper}");

    lower - upper
}

#[derive(Debug, Default)]
pub struct Challenge;

impl Challenge {
    fn invalid_id(id: u64) -> bool {
        let half_num_digits = (id.ilog10() + 1) / 2;
        let top_half = decimal_rsh(id, half_num_digits);
        let bottom_half = id - decimal_lsh(top_half, half_num_digits);

        top_half == bottom_half
    }

    fn invalid_id_any(id: u64) -> bool {
        let num_digits = id.ilog10() + 1;
        for i in 1..=(num_digits / 2) {
            if num_digits % i != 0 {
                continue;
            }

            let num_reps = num_digits / i;
            let all_equal = (0..num_reps)
                .map(|j| decimal_substr(id, j * i, (j + 1) * i))
                .all_equal();

            if all_equal {
                return true;
            }
        }

        false
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let re = Regex::new("(\\d+)-(\\d+),?").unwrap();

        let (ids_1, ids_2) = re
            .captures_iter(input)
            .map(|m| m.extract::<2>().1)
            .flat_map(|[start, end]| {
                u64::from_str_radix(start, 10).unwrap()..u64::from_str_radix(end, 10).unwrap()
            })
            .tee();

        let total_of_invalid_ids: u64 = ids_1.filter(|id| Self::invalid_id(*id)).sum();
        let total_of_invalid_ids_any: u64 = ids_2
            .filter(|id| Self::invalid_id_any(*id))
            .map(|id| {
                println!("{id}");
                id
            })
            .sum();

        Ok(AOCResult {
            part_1: total_of_invalid_ids.to_string(),
            part_2: total_of_invalid_ids_any.to_string(),
        })
    }
}
