use anyhow::{anyhow, Result};
use std::{env::args, fmt::Debug, fs::read_to_string};

fn get_filepath() -> Result<String> {
  args()
    .skip(1)
    .next()
    .ok_or(anyhow!("Input file path not specified"))
}

fn execute<T1, T2>(solution: T1) -> Result<T2>
where
  T1: Fn(String) -> Result<T2>,
  T2: Debug,
{
  let filepath = get_filepath()?;
  let input = read_to_string(&filepath)?;
  solution(input)
}

pub fn run<T1, T2>(solution: T1)
where
  T1: Fn(String) -> Result<T2>,
  T2: Debug,
{
  match execute(solution) {
    Ok(value) => println!("Answer: {:?}", value),
    Err(error) => {
      eprintln!("Error: {:?}", error);
      std::process::exit(1);
    }
  }
}
