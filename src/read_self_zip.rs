use crate::SAVEFILE_VERSION;
use lazy_static::lazy_static;
use savefile::prelude::*;

use crate::schedule::Catalog;

lazy_static! {
    pub static ref CATALOGS: Vec<Catalog> = {
        let x: Vec<Catalog> =
            load_from_mem(include_bytes!("../assets/catalogs.bin"), SAVEFILE_VERSION).unwrap();
        x
    };
}
