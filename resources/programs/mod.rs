use crate::schedule::Program;
mod prog_;
mod prog_acctba;
mod prog_arch_ba;
mod prog_art_ba;
mod prog_astr_bs;
mod prog_athlhc_ba;
mod prog_bio_ba;
mod prog_bio_bs;
mod prog_biochem_ba;
mod prog_biochem_bs;
mod prog_chem_ba;
mod prog_chem_bs;
mod prog_cheng_bs;
mod prog_cive_bs;
mod prog_classl_ba;
mod prog_classlg_ba;
mod prog_classlgh_ba;
mod prog_compsci_ba;
mod prog_compsci_bs;
mod prog_crim_ba;
mod prog_econ_ba;
mod prog_eled_ba;
mod prog_eleng_bs;
mod prog_phys_ba;

pub fn programs() -> Vec<Program> {
    vec![
        prog_::prog(),
        prog_acctba::prog(),
        prog_arch_ba::prog(),
        prog_art_ba::prog(),
        prog_astr_bs::prog(),
        prog_athlhc_ba::prog(),
        prog_biochem_ba::prog(),
        prog_biochem_bs::prog(),
        prog_bio_ba::prog(),
        prog_bio_bs::prog(),
        prog_chem_ba::prog(),
        prog_chem_bs::prog(),
        prog_cheng_bs::prog(),
        prog_cive_bs::prog(),
        prog_classlgh_ba::prog(),
        prog_classlg_ba::prog(),
        prog_classl_ba::prog(),
        prog_compsci_ba::prog(),
        prog_compsci_bs::prog(),
        prog_crim_ba::prog(),
        prog_econ_ba::prog(),
        prog_eled_ba::prog(),
        prog_eleng_bs::prog(),
        prog_phys_ba::prog(),
    ]
}
