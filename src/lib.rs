pub type Day = i8;

#[derive(Debug, PartialEq)]
pub enum Solution {
    I32(i32),
    I64(i64),
    I128(i128),
    U32(u32),
    U64(u64),
    U128(u128),
    Str(String),
}

use std::fmt::{Display, Formatter, Result};
impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Solution::I32(x) => x.fmt(f),
            Solution::I64(x) => x.fmt(f),
            Solution::I128(x) => x.fmt(f),
            Solution::U32(x) => x.fmt(f),
            Solution::U64(x) => x.fmt(f),
            Solution::U128(x) => x.fmt(f),
            Solution::Str(x) => x.fmt(f),
        }
    }
}

pub mod load;
pub use load::get_data_server;

pub mod solutions;
pub use solutions::solve;
