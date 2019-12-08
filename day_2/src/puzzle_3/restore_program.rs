use shared::intcode::Program;

/// A quote from problem statement:
/// > Once you have a working computer, the first step is to restore
/// > the gravity assist program (your puzzle input) to the "1202 program alarm"
/// > state it had just before the last computer caught fire. To do this,
/// > before running the program, replace position 1 with the value 12
/// > and replace position 2 with the value 2.
pub fn restore_program(program: &mut Program) {
  // assume that the program has at least three symbols
  program.legacy_set_input(12, 2);
}
