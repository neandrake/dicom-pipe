pub mod dcmparser;
pub mod tagstop;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use encoding::EncodingRef;

pub type CSRef = EncodingRef;
