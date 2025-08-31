use crate::schedule::Program;
mod prog_acctba;
mod prog_arch_ba;
mod prog_art_ba;
mod prog_astr_bs;
mod prog_athlhc_ba;
mod prog_biochem_ba;
mod prog_biochem_bs;
mod prog_chem_ba;
mod prog_phys_ba;

pub fn programs() -> Vec<Program> {
    vec![
        prog_acctba::prog(),
        prog_arch_ba::prog(),
        prog_art_ba::prog(),
        prog_astr_bs::prog(),
        prog_athlhc_ba::prog(),
        prog_biochem_ba::prog(),
        prog_biochem_bs::prog(),
        prog_chem_ba::prog(),
        prog_phys_ba::prog(),
    ]
}
