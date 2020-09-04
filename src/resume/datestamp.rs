use serde::{Serialize, Deserialize, Serializer, Deserializer, self};
// TODO: Tidy module imports

#[doc = ""] // TODO Write documentation
#[derive(Copy, Clone, Debug)]
pub enum Datestamp {
    Year (i32),
    YearMonth(i32, u32),
    YearMonthDay(i32, u32, u32)
}

impl Serialize for Datestamp {
    fn serialize<S>(&self, serializer: S)
        -> Result<S::Ok, S::Error> where S: Serializer {
            match *self {
                Datestamp::Year(y) => {
                    serializer.serialize_i32(y);
                    Ok(S::Ok()) // FIXME: no associated item named `Ok` found for type parameter `S` in the current scope rustc(E0599)
                },

                Datestamp::YearMonth(y, m) => {
                    serializer.serialize_i32(y);
                    serializer.serialize_u32(m);
                    Ok(S::Ok) // FIXME: no associated item named `Ok` found for type parameter `S` in the current scope rustc(E0599)
                },

                Datestamp::YearMonthDay(y, m, d) => {
                    serializer.serialize_i32(y);
                    serializer.serialize_u32(m);
                    serializer.serialize_u32(d);
                    Ok(S::Ok) // FIXME: no associated item named `Ok` found for type parameter `S` in the current scope rustc(E0599)
                }
            }
    }
}

impl<'de> Deserialize<'de> for Datestamp {

    fn deserialize<D>(deserializer: D)
        -> Result<Self, D::Error> where D: Deserializer<'de> {
            todo!()
    }
}

use std::fmt::{Formatter as FmtFormatter, Result as FmtResult};
use regex::Regex;
use lazy_static::lazy_static;
use serde::de::{Visitor, Error as DeError};

lazy_static! {
    static ref REGEXP: Regex = {
        Regex::new("").unwrap()
    };
}

impl<'de> Visitor<'de> for Datestamp {
    type Value = Self;

    fn expecting(&self, formatter: &mut FmtFormatter) -> FmtResult {
        formatter.write_str("a date, which consists of at least a year, and optionally a month and day")
    }

    fn visit_str<E>(self, value: &str)
        -> Result<<Self as Visitor>::Value, E> where E: DeError {
            Err(E::custom("todo!()"))
    }
}
