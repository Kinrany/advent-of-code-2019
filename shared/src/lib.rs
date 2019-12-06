pub mod intcode;

use anyhow::{anyhow, Result};
use std::{env::args, fmt::Debug, fs::read_to_string};

pub fn get_first_arg() -> Result<String> {
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
  let filepath = get_first_arg()?;
  let input = read_to_string(&filepath)?;
  solution(input)
}

pub fn print<T>(result: Result<T>)
where
  T: Debug,
{
  match result {
    Ok(value) => println!("Answer: {:?}", value),
    Err(error) => {
      eprintln!("Error: {:?}", error);
      std::process::exit(1);
    }
  }
}

pub fn run<T1, T2>(solution: T1)
where
  T1: Fn(String) -> Result<T2>,
  T2: Debug,
{
  print(execute(solution));
}
