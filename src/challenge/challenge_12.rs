use anyhow::anyhow;

use super::{AOCChallenge, AOCResult};

#[derive(Debug, Default)]
pub struct Challenge;

impl AOCChallenge for Challenge {
    fn run(self, _input: &str) -> anyhow::Result<AOCResult> {
        Err(anyhow!("Not yet implemented"))
    }
}
