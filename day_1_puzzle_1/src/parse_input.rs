pub fn parse_input(input: &String) -> anyhow::Result<Vec<i64>> {
  let numbers: Result<Vec<_>, _> = input.lines().map(&str::parse::<i64>).collect();
  numbers.map_err(anyhow::Error::new)
}
