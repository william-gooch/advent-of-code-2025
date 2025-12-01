use anyhow::anyhow;
use regex::Regex;

use super::{AOCChallenge, AOCResult};

#[derive(Debug, Default)]
pub struct Challenge;

#[derive(PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Direction not matched.")),
        }
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let re = Regex::new("([LR])(\\d+)").unwrap();

        let (final_angle, end_zero_count, total_zero_count) = input
            .lines()
            .map(|line| re.captures(line).unwrap().extract::<2>().1)
            .try_fold(
                (50, 0, 0),
                |(mut old_angle, mut end_zero_count, mut total_zero_count),
                 [direction, turn]|
                 -> anyhow::Result<(i16, u32, u32)> {
                    println!("{direction} {turn}");
                    let direction = Direction::try_from(direction)?;
                    let turn = u16::from_str_radix(turn, 10)?;

                    let mut new_angle = match direction {
                        Direction::Left => old_angle
                            .checked_sub_unsigned(turn)
                            .ok_or(anyhow!("Underflow error"))?,
                        Direction::Right => old_angle
                            .checked_add_unsigned(turn)
                            .ok_or(anyhow!("Overflow error"))?,
                    };

                    total_zero_count += new_angle.div_euclid(100).abs() as u32;
                    if new_angle < 0 && old_angle == 0 {
                        total_zero_count -= 1;
                    }
                    new_angle = new_angle.rem_euclid(100);

                    if new_angle == 0 {
                        end_zero_count += 1;
                        if direction == Direction::Left {
                            total_zero_count += 1;
                        }
                    }

                    println!("> {old_angle} => {new_angle} {total_zero_count}");
                    Ok((new_angle, end_zero_count, total_zero_count))
                },
            )?;

        Ok(AOCResult::new(
            end_zero_count.to_string(),
            total_zero_count.to_string(),
        ))
    }
}
