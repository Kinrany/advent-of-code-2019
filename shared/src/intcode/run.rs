use super::program::{ExecutionResult, Program};

pub fn run_with_input<T>(program: &mut Program, input: &mut T) -> Vec<isize>
where
  T: Iterator<Item = isize>,
{
  use ExecutionResult::*;

  let mut output = Vec::new();

  let mut halted = false;
  let mut ptr = 0;
  while !halted {
    let op = program.get_op_at(ptr).expect("Invalid program");
    let result = program.execute_op(op, input);
    match result {
      Halt => {
        halted = true;
      }
      Continue { ptr_offset } => {
        ptr += ptr_offset;
      }
      Output { ptr_offset, value } => {
        ptr += ptr_offset;
        output.push(value);
      }
    }
  }

  output
}

pub fn run(program: &mut Program) -> Vec<isize> {
  let mut input = Vec::new().into_iter();
  run_with_input(program, &mut input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_0() {
    let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].into();
    let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50].into();

    run(&mut program);

    assert_eq!(program, expected);
  }

  #[test]
  fn example_1() {
    let mut program = vec![1, 0, 0, 0, 99].into();
    let expected = vec![2, 0, 0, 0, 99].into();

    run(&mut program);

    assert_eq!(program, expected);
  }

  #[test]
  fn example_2() {
    let mut program = vec![2, 3, 0, 3, 99].into();
    let expected = vec![2, 3, 0, 6, 99].into();

    run(&mut program);

    assert_eq!(program, expected);
  }

  #[test]
  fn example_3() {
    let mut program = vec![2, 4, 4, 5, 99, 0].into();
    let expected = vec![2, 4, 4, 5, 99, 9801].into();

    run(&mut program);

    assert_eq!(program, expected);
  }

  #[test]
  fn example_4() {
    let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99].into();
    let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99].into();

    run(&mut program);

    assert_eq!(program, expected);
  }

  #[test]
  fn io_example_1() {
    let input = vec![42];
    let expected_output = vec![42];

    let mut program = vec![3, 0, 4, 0, 99].into();
    let expected_program = vec![42, 0, 4, 0, 99].into();

    let output = run_with_input(&mut program, &mut input.into_iter());

    assert_eq!(program, expected_program);
    assert_eq!(output, expected_output);
  }

  #[test]
  fn example_5() {
    let mut program = vec![1002, 4, 3, 4, 33].into();
    let expected_program = vec![1002, 4, 3, 4, 99].into();

    run(&mut program);

    assert_eq!(program, expected_program);
  }

  #[test]
  fn example_6() {
    let mut program = vec![1101, 100, -1, 4, 0].into();
    let expected_program = vec![1101, 100, -1, 4, 99].into();

    run(&mut program);

    assert_eq!(program, expected_program);
  }
}
