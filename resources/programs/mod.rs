use crate::schedule::Program;
mod prog_acct_ba;
mod prog_arch_ba;
mod prog_chem_ba;
mod prog_phys_ba;

pub fn programs() -> Vec<Program> {
    vec![
        prog_acct_ba::prog(),
        prog_arch_ba::prog(),
        prog_chem_ba::prog(),
        prog_phys_ba::prog(),
    ]
}
