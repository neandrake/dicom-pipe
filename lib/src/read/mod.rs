pub mod dcmdataset;
pub mod dcmelement;
pub mod dcmiterator;
pub mod tagstop;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use encoding::EncodingRef;

pub type CSRef = EncodingRef;
