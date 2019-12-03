use day_2_puzzle_3::{run_program, Program};

pub fn run_program_with_input(program: &Program, input1: usize, input2: usize) -> usize {
  let mut program = program.clone();
  program[1] = input1;
  program[2] = input2;
  run_program(&mut program);
  program[0]
}
