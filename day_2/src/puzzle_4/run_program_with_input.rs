use shared::intcode::{run, Program};

pub fn run_program_with_input(program: &Program, input1: usize, input2: usize) -> usize {
  let mut program = program.clone();
  program[1] = input1;
  program[2] = input2;
  run(&mut program);
  program[0]
}
