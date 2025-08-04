use crate::schedule::Program;
use lazy_static::lazy_static;
use std::collections::HashMap;

mod prog_acct;

lazy_static! {
    pub static ref PROGRAMS_MAP: HashMap<String, Program> = {
        let mut m = HashMap::new();
        let x = prog_acct::prog(); m.insert(x.name.clone(), x);
        m
    };
}
