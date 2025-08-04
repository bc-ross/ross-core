use crate::schedule::Program;
mod prog_acct;
mod prog_arch;
mod prog_bsee;

pub fn programs() -> Vec<Program> {
    vec![prog_acct::prog(), prog_arch::prog(), prog_bsee::prog()]
}
