#[link(name = "math", vers = "0.0", package_id = "math")];
#[crate_type = "lib"];

#[feature(globs)];

extern mod data;

pub mod arith;
pub mod cont_frac;
pub mod numconv;
pub mod poly;
pub mod prime;
pub mod sequence;
