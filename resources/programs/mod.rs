use crate::schedule::Program;
use lazy_static::lazy_static;
use std::collections::HashMap;
mod prog_acct;
mod prog_arch;

lazy_static! {
    pub static ref PROGRAMS_MAP: HashMap<String, Program> = {
        let mut m = HashMap::new();
        let x = prog_acct::prog();
        m.insert(x.name.clone(), x);
        let x = prog_arch::prog();
        m.insert(x.name.clone(), x);
        m
    };
}
