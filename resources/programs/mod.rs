use crate::schedule::Program;
mod prog_acctba;
mod prog_chem_ba;
mod prog_phys_ba;

pub fn programs() -> Vec<Program> {
    vec![
        prog_acctba::prog(),
        prog_chem_ba::prog(),
        prog_phys_ba::prog(),
    ]
}
