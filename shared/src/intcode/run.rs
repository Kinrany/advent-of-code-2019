use super::program::{ExecutionResult, Program};

pub fn run(program: &mut Program) {
  use ExecutionResult::*;

  let mut halted = false;
  let mut ptr = 0;
  while !halted {
    let op = program.get_op_at(ptr).expect("Invalid program");
    let result = program.execute_op(op);
    match result {
      Halt => {
        halted = true;
      }
      Continue { ptr_offset } => {
        ptr += ptr_offset;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::run;

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
}
