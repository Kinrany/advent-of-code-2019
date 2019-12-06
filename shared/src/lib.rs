pub mod input;
pub mod intcode;

use anyhow::Result;
use std::fmt::Debug;

fn execute<T1, T2>(solution: T1) -> Result<T2>
where
  T1: Fn(String) -> Result<T2>,
  T2: Debug,
{
  let input = input::string()?;
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
