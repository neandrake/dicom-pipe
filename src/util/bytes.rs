#![allow(dead_code)]

extern crate byteorder;

//use byteorder::{BigEndian,LittleEndian};

pub fn u32_to_bytes_le(value: u32, bytes: &mut Vec<u8>, bytes_offset: usize) {
    bytes[bytes_offset] =  (value & 0xFF) as u8;
    bytes[bytes_offset + 1] = ((value >> 8) & 0xFF) as u8;
    bytes[bytes_offset + 2] = ((value >> 16) & 0xFF) as u8;
    bytes[bytes_offset + 3] = ((value >> 24) & 0xFF) as u8;  
} 

pub fn u32s_to_bytes_le(value: &mut Vec<u32>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(value.len() << 2);
    for i in 0..value.len() {
        u32_to_bytes_le(value[i], &mut bytes, i << 2);
    }
    bytes
}

pub fn u32_to_bytes_be(value: u32, bytes: &mut Vec<u8>, bytes_offset: usize) {
    bytes[bytes_offset] = ((value >> 24) & 0xFF) as u8;
    bytes[bytes_offset + 1] = ((value >> 16) & 0xFF) as u8;
    bytes[bytes_offset + 2] = ((value >> 8) & 0xFF) as u8;
    bytes[bytes_offset + 3] = (value & 0xFF) as u8;
}

pub fn u32s_to_bytes_be(value: &mut Vec<u32>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(value.len() << 2);
    for i in 0..value.len() {
        u32_to_bytes_be(value[i], &mut bytes, i << 2);
    }
    bytes
}

pub fn bytes_le_to_u32(bytes: &mut Vec<u8>, bytes_offset: usize) -> u32 {
    (((bytes[bytes_offset + 3] & 0xFF) as u32) << 24)
    + (((bytes[bytes_offset + 2] & 0xFF) as u32) << 16)
    + (((bytes[bytes_offset + 1] & 0xFF) as u32) << 8)
    + (((bytes[bytes_offset] & 0xFF) as u32))
}

pub fn bytes_le_to_u32s(bytes: &mut Vec<u8>) -> Vec<u32> {
    if bytes.len() & 0x3 != 0 {
        // return error
    }

    let mut ints: Vec<u32> = Vec::with_capacity(bytes.len() >> 2);
    for i in 0..ints.len() {
        ints[i] = bytes_le_to_u32(bytes, i << 2);
    }
    ints
}

pub fn bytes_be_to_u32(bytes: &mut Vec<u8>, bytes_offset: usize) -> u32 {
    (((bytes[bytes_offset] & 0xFF) as u32) << 24)
    + (((bytes[bytes_offset + 1] & 0xFF) as u32) << 16)
    + (((bytes[bytes_offset + 2] & 0xFF) as u32) << 8)
    + (((bytes[bytes_offset] + 3) & 0xFF) as u32)
}

pub fn bytes_be_to_u32s(bytes: &mut Vec<u8>) -> Vec<u32> {
    if bytes.len() & 0x3 != 0 {
        // return error
    }

    let mut ints: Vec<u32> = Vec::with_capacity(bytes.len() >> 2);
    for i in 0..ints.len() {
        ints[i] = bytes_be_to_u32(bytes, i << 2);
    }
    ints
}

pub fn tag_to_bytes_le(tag: u32, bytes: &mut Vec<u8>, bytes_offset: usize) {
    bytes[bytes_offset] = ((tag >> 16) & 0xFF) as u8;
    bytes[bytes_offset + 1] = ((tag >> 24) & 0xFF) as u8;
    bytes[bytes_offset + 2] = (tag & 0xFF) as u8;
    bytes[bytes_offset + 3] = ((tag >> 8) & 0xFF) as u8;
}

pub fn tags_to_bytes_le(tags: &mut Vec<u32>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(tags.len() << 2);
    for i in 0..tags.len() {
        tag_to_bytes_le(tags[i], &mut bytes, i << 2);
    }
    bytes
}

pub fn tag_to_bytes_be(tag: u32, bytes: &mut Vec<u8>, bytes_offset: usize) {
    u32_to_bytes_be(tag, bytes, bytes_offset)
}

pub fn tags_to_bytes_be(tags: &mut Vec<u32>) -> Vec<u8> {
    u32s_to_bytes_be(tags)
}

pub fn bytes_le_to_tag(bytes: &mut Vec<u8>, bytes_offset: usize) -> u32 {
    (((bytes[bytes_offset + 1] & 0xFF) as u32) << 24)
    + (((bytes[bytes_offset] & 0xFF) as u32) << 16)
    + (((bytes[bytes_offset + 3] & 0xFF) as u32) << 8)
    + (((bytes[bytes_offset + 2] & 0xFF) as u32))
}

pub fn bytes_le_to_tags(bytes: &mut Vec<u8>) -> Vec<u32> {
    if bytes.len() & 0x3 != 0 {
        // return error
    }

    let mut tags: Vec<u32> = Vec::with_capacity(bytes.len() >> 2);
    for i in 0..tags.len() {
        tags[i] = bytes_le_to_tag(bytes, i << 2);
    }
    tags
}

pub fn bytes_be_to_tag(bytes: &mut Vec<u8>, bytes_offset: usize) -> u32 {
    bytes_be_to_u32(bytes, bytes_offset)
}

pub fn bytes_be_to_tags(bytes: &mut Vec<u8>) -> Vec<u32> {
    bytes_be_to_u32s(bytes)
}

pub fn u16_to_bytes_le(value: u16, bytes: &mut Vec<u8>, bytes_offset: usize) {
    bytes[bytes_offset] = (value & 0xFF) as u8;
    bytes[bytes_offset + 1] = (value >> 8) as u8;
}

pub fn u16s_to_bytes_le(value: &mut Vec<u16>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(value.len() << 1);
    for i in 0..value.len() {
        u16_to_bytes_le(value[i], &mut bytes, i << 1);
    }
    bytes
}

pub fn u16_to_bytes_be(value: u16, bytes: &mut Vec<u8>, bytes_offset: usize) {
    bytes[bytes_offset] = (value >> 8) as u8;
    bytes[bytes_offset + 1] = (value & 0xFF) as u8;
}

pub fn u16s_to_bytes_be(value: &mut Vec<u16>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(value.len() << 1);
    for i in 0..value.len() {
        u16_to_bytes_be(value[i], &mut bytes, i << 1);
    }
    bytes
}
