//#![allow(dead_code)]
#![allow(non_camel_case_types)]

//! Value Multiplicity

#[derive(Debug, PartialEq, Eq)]
pub enum VM {
    /// A set number of items: 1, 2, 3, 4, 6, 9, 16, etc.
    Distinct(u32),
    /// A minimum number but possibly more: 1-n, 2-n, 3-n, 6-n, etc.
    AtLeast(u32),
    /// A maximum number but at least one: 1-2, 1-3, 1-32, 1-99, etc.
    AtMost(u32),
    /// A multiple of some number: 2-2n, 3-3n, etc.
    MultipleOf(u32),
    /// Single or multiple: 1-n or 1
    OneOrMore,
}
