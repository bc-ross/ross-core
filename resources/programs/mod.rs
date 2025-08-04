use crate::schedule::Program;
mod prog_acct;
mod prog_arch;

pub fn programs() -> Vec<Program> {
    vec![prog_acct::prog(), prog_arch::prog()]
}
