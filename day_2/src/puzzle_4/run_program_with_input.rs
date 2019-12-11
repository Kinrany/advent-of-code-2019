use shared::intcode::{run, Program};

pub fn run_program_with_input(program: &Program, in1: isize, in2: isize) -> isize {
  let mut program = program.clone();
  program.legacy_set_input(in1, in2);
  run(&mut program);
  program.legacy_get_output()
}
