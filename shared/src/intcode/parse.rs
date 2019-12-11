use super::Program;

pub fn parse(input: &str) -> anyhow::Result<Program> {
  input
    .trim()
    .split(',')
    .map(&str::parse::<isize>)
    .collect::<Result<Program, _>>()
    .map_err(anyhow::Error::new)
}
