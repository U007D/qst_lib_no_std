use std::{
    ffi::OsString,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
};

use crate::consts::*;

#[derive(Debug)]
pub enum Error {
    ArgNotConvertibleToUtf8(OsString),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Error::ArgNotConvertibleToUtf8(os_string) => format!("{}: {:?}",
                                                                 msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8,
                                                                 os_string),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::ArgNotConvertibleToUtf8(err)
    }
}
