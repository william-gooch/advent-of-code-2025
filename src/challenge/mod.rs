use std::{fmt::Display, io::Read, path::Path};

use anyhow::Result;

mod challenge_1;
mod challenge_10;
mod challenge_11;
mod challenge_12;
mod challenge_2;
mod challenge_3;
mod challenge_4;
mod challenge_5;
mod challenge_6;
mod challenge_7;
mod challenge_8;
mod challenge_9;

pub use challenge_1::Challenge as Challenge1;
pub use challenge_2::Challenge as Challenge2;
pub use challenge_3::Challenge as Challenge3;
pub use challenge_4::Challenge as Challenge4;
pub use challenge_5::Challenge as Challenge5;
pub use challenge_6::Challenge as Challenge6;
pub use challenge_7::Challenge as Challenge7;
pub use challenge_8::Challenge as Challenge8;
pub use challenge_9::Challenge as Challenge9;
pub use challenge_10::Challenge as Challenge10;
pub use challenge_11::Challenge as Challenge11;
pub use challenge_12::Challenge as Challenge12;

pub struct AOCResult {
    part_1: String,
    part_2: String,
}

impl AOCResult {
    pub fn new(part_1: impl Into<String>, part_2: impl Into<String>) -> AOCResult {
        AOCResult {
            part_1: part_1.into(),
            part_2: part_2.into(),
        }
    }
}

impl Display for AOCResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Part 1: {}, Part 2: {}", self.part_1, self.part_2)
    }
}

pub trait AOCChallenge {
    fn run(self, input: &str) -> Result<AOCResult>;

    fn run_file(self, path: &Path) -> Result<AOCResult>
    where
        Self: Sized,
    {
        let file: String = std::fs::read_to_string(path)?;

        self.run(file.as_str())
    }
}
