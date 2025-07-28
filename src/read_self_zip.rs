use crate::SAVEFILE_VERSION;
use anyhow::anyhow;
use lazy_static::lazy_static;
use num_enum::TryFromPrimitive;
use polars::functions::concat_df_horizontal;
use polars::prelude::*;
use quick_xml::de::from_str;
use rc_zip_sync::ReadZip;
use savefile::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::path;
use struct_field_names_as_array::FieldNamesAsArray;
use strum_macros::EnumString;

use crate::schedule::Catalog;

lazy_static! {
    pub static ref CATALOGS: Vec<Catalog> = {
        let x: Vec<Catalog> =
            load_from_mem(include_bytes!("../assets/catalogs.bin"), SAVEFILE_VERSION).unwrap();
        x
    };
}
