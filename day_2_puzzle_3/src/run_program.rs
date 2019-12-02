use super::Program;
use itertools::Itertools;

pub fn run_program(program: &mut Program) {
  let mut ptr = 0;
  loop {
    match program[ptr] {
      99 => break,
      1 => {
        let (&input1, &input2, &output) = program
          .iter()
          .skip(ptr + 1)
          .next_tuple()
          .expect("program to be valid");
        program[output] = program[input1] + &program[input2];
      }
      2 => {
        let (&input1, &input2, &output) = program
          .iter()
          .skip(ptr + 1)
          .next_tuple()
          .expect("program to be valid");
        program[output] = program[input1] * &program[input2];
      }
      _ => panic!("Invalid program"),
    };
    ptr += 4;
  }
}

#[cfg(test)]
mod tests {
  use super::run_program;

  #[test]
  fn example_0() {
    let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected_output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

    let mut output = input;
    run_program(&mut output);

    assert_eq!(output, expected_output);
  }

  #[test]
  fn example_1() {
    let input = vec![1, 0, 0, 0, 99];
    let expected_output = vec![2, 0, 0, 0, 99];

    let mut output = input;
    run_program(&mut output);

    assert_eq!(output, expected_output);
  }

  #[test]
  fn example_2() {
    let input = vec![2, 3, 0, 3, 99];
    let expected_output = vec![2, 3, 0, 6, 99];

    let mut output = input;
    run_program(&mut output);

    assert_eq!(output, expected_output);
  }

  #[test]
  fn example_3() {
    let input = vec![2, 4, 4, 5, 99, 0];
    let expected_output = vec![2, 4, 4, 5, 99, 9801];

    let mut output = input;
    run_program(&mut output);

    assert_eq!(output, expected_output);
  }

  #[test]
  fn example_4() {
    let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let expected_output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

    let mut output = input;
    run_program(&mut output);

    assert_eq!(output, expected_output);
  }
}
