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
      Continue { ptr_set } => {
        ptr = ptr_set(ptr);
      }
      Output { ptr_set, value } => {
        ptr = ptr_set(ptr);
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

  fn test_program_output<T1, T2>(program: T1, tests: T2)
  where
    T1: Into<Program>,
    T2: IntoIterator<Item = (Vec<isize>, Vec<isize>)>,
  {
    let program: Program = program.into();
    tests.into_iter().for_each(|(input, expected_output)| {
      let mut program = program.clone();
      let output = run_with_input(&mut program, &mut input.into_iter());
      assert_eq!(output, expected_output);
    });
  }

  #[test]
  fn io_example_1() {
    test_program_output(vec![3, 0, 4, 0, 99], vec![(vec![42], vec![42])]);
  }

  /// puzzle 10
  #[test]
  fn io_example_2() {
    test_program_output(
      vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
      vec![
        (vec![6], vec![0]),
        (vec![7], vec![0]),
        (vec![8], vec![1]),
        (vec![9], vec![0]),
        (vec![10], vec![0]),
      ],
    );
  }

  /// puzzle 10
  #[test]
  fn io_example_3() {
    test_program_output(
      vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
      vec![
        (vec![6], vec![1]),
        (vec![7], vec![1]),
        (vec![8], vec![0]),
        (vec![9], vec![0]),
        (vec![10], vec![0]),
      ],
    );
  }

  /// puzzle 10
  #[test]
  fn io_example_4() {
    test_program_output(
      vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
      vec![
        (vec![6], vec![0]),
        (vec![7], vec![0]),
        (vec![8], vec![1]),
        (vec![9], vec![0]),
        (vec![10], vec![0]),
      ],
    );
  }

  /// puzzle 10
  #[test]
  fn io_example_5() {
    test_program_output(
      vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
      vec![
        (vec![6], vec![1]),
        (vec![7], vec![1]),
        (vec![8], vec![0]),
        (vec![9], vec![0]),
        (vec![10], vec![0]),
      ],
    );
  }

  /// puzzle 10
  #[test]
  fn io_example_6() {
    test_program_output(
      vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
      vec![
        (vec![0], vec![0]),
        (vec![1], vec![1]),
        (vec![2], vec![1]),
        (vec![99], vec![1]),
        (vec![-99], vec![1]),
      ],
    );
  }

  /// puzzle 10
  #[test]
  fn io_example_7() {
    test_program_output(
      vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
      vec![
        (vec![0], vec![0]),
        (vec![1], vec![1]),
        (vec![2], vec![1]),
        (vec![99], vec![1]),
        (vec![-99], vec![1]),
      ],
    );
  }

  /// puzzle 10
  #[test]
  fn io_example_8() {
    test_program_output(
      vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
      ],
      vec![
        (vec![6], vec![999]),
        (vec![7], vec![999]),
        (vec![8], vec![1000]),
        (vec![9], vec![1001]),
        (vec![10], vec![1001]),
      ],
    );
  }
}
