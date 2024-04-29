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

use self::{
    constants::tags::ITEM,
    vl::ValueLength,
    vr::{VRRef, OB, OF, OW, UN},
};

pub mod constants;
pub mod dcmdict;
pub mod tag;
pub mod ts;
pub mod uid;
pub mod vl;
pub mod vm;
pub mod vr;

/// Whether the element is a non-standard parent-able element. These are non-SQ, non-ITEM elements
/// with a VR of `UN`, `OB`, `OF`, or `OW` and have a value length of `UndefinedLength`. These
/// types of elements are considered either private-tag sequences or otherwise whose contents are
/// encoded as IVRLE.
pub(crate) fn is_non_standard_sq<T>(tag: T, vr: VRRef, vl: ValueLength) -> bool
where
    u32: From<T>,
{
    let tag: u32 = u32::from(tag);
    tag != ITEM
        && (vr == &UN || vr == &OB || vr == &OF || vr == &OW)
        && vl == ValueLength::UndefinedLength
}
