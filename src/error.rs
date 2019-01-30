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
    fn fmt(&self, _f: &mut Formatter) -> FmtResult {
        unimplemented!()
    }
}
