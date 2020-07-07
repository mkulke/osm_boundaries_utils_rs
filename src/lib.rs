extern crate geo_types;
#[macro_use]
extern crate log;
extern crate geo;
extern crate osmpbfreader;
extern crate smol_str;

mod boundaries;
pub mod osm_builder;

pub use boundaries::build_boundary;
