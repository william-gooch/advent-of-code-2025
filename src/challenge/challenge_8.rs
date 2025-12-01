use super::AOCChallenge;

#[derive(Debug, Default)]
pub struct Challenge;

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<String> {
        Ok("".to_string())
    }
}
