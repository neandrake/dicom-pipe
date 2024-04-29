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

use crate::core::{
    dcmsqelem::SequenceElement,
    defn::{
        constants::tags::{ITEM, ITEM_DELIMITATION_ITEM, SEQUENCE_DELIMITATION_ITEM},
        tag::Tag,
        vl::ValueLength,
        vr::{VRRef, OB, OF, OW, UN},
    },
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
/// encoded as Implicit VR.
pub(crate) fn is_non_standard_sq<T>(tag: T, vr: VRRef, vl: ValueLength) -> bool
where
    u32: From<T>,
{
    let tag: u32 = u32::from(tag);
    tag != ITEM
        && (vr == &UN || vr == &OB || vr == &OF || vr == &OW)
        && vl == ValueLength::UndefinedLength
}

/// There are three special SQ related Data Elements that are not ruled by the VR encoding
/// rules conveyed by the Transfer Syntax. They shall be encoded as Implicit VR. These
/// special Data Elements are Item (FFFE,E000), Item Delimitation Item (FFFE,E00D), and
/// Sequence Delimitation Item (FFFE,E0DD). However, the Data Set within the Value Field of
/// the Data Element Item (FFFE,E000) shall be encoded according to the rules conveyed by the
/// Transfer Syntax.
/// (Part 5, Section 7.5)
pub(crate) fn is_sq_delim<T>(tag: T) -> bool
where
    u32: From<T>,
{
    let tag = u32::from(tag);
    tag == SEQUENCE_DELIMITATION_ITEM || tag == ITEM_DELIMITATION_ITEM || tag == ITEM
}

/// Given the path to an element, determines if the element would be considered in a private-tag
/// sequence, indicating the element should be encoded as Implicit VR instead of the dataset's
/// transfer syntax.
///
/// # Notes
/// See: Part 5, Section 6.2.2
/// Elements within a Private Sequence with VR of UN should be in Implicit VR.
/// Elements within a Private Sequence with VR of SQ and VL of Undefined should use the
///   Dataset Transfer Syntax.
/// XXX: ?? Elements within a Private Sequence with VR of SQ and VL of Explicit should be in
///   Implicit VR.
pub(crate) fn is_parent_priv_sq(sq_path: &[SequenceElement]) -> bool {
    sq_path.iter().rev().any(|sq_el| {
        Tag::is_private(sq_el.sq_tag())
            && is_non_standard_sq(sq_el.sq_tag(), sq_el.vr(), sq_el.vl())
    })
}
