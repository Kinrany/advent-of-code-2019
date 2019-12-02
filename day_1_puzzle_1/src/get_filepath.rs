use anyhow::{anyhow, Result};

pub fn get_filepath() -> Result<String> {
  std::env::args()
    .skip(1)
    .next()
    .ok_or(anyhow!("Input file path not specified"))
}
