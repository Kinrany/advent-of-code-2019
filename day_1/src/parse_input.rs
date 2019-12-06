pub fn parse_input(input: &str) -> anyhow::Result<Vec<i64>> {
  input
    .lines()
    .map(&str::parse::<i64>)
    .collect::<Result<Vec<_>, _>>()
    .map_err(anyhow::Error::new)
}
