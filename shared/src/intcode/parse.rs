use super::Program;

pub fn parse(input: &str) -> anyhow::Result<Program> {
  input
    .trim()
    .split(',')
    .map(&str::parse::<usize>)
    .collect::<Result<Vec<_>, _>>()
    .map_err(anyhow::Error::new)
}