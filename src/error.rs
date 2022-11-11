use core::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
};

use crate::consts::*;

#[derive(Debug)]
pub enum Error {}

impl Display for Error {
    // TODO: Reimplement https://github.com/zartarn15/format_no_std/blob/master/src/lib.rs as `he_std` for `no_std` `format` capability
    fn fmt(&self, _f: &mut Formatter) -> FmtResult {
        unimplemented!()
    }
}
