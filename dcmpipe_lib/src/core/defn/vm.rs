/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! Value Multiplicity

pub type VMRef = &'static VM;

/// Value Multiplicity Definition
///
/// Part 5, Ch 6.4:
/// Data Elements with a VR of OB, OD, OF, OL, OW, OB, SQ, UN or UR
/// shall always have a Value Multiplicity of one.
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

    /// Single or multiple: 1 or 1-n
    OneOrMore,
}
