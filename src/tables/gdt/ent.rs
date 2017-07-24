use super::*;

/// The first descriptor in GDT is null.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct NullDescriptor {
    null    : u64
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct CallGateDescriptor {
    offset0 : u16,
    segsel  : u16,
    flags   : u16,
    offset1 : u16,
    offset2 : u32,
    resv    : u32,
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TssDescriptor {
    limit   : u16,
    base0   : u16,
    flags0  : u16,
    flags1  : u8 ,
    base1   : u8 ,
    base2   : u32,
    resv    : u32,
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LdtDescriptor {
    limit   : u16,
    base0   : u16,
    flags0  : u16,
    flags1  : u8 ,
    base1   : u8 ,
    base2   : u32,
    resv    : u32,
}

impl Entry for NullDescriptor {
}

impl Entry for CallGateDescriptor {
}

impl Entry for TssDescriptor {
}

impl Entry for LdtDescriptor {
}

pub mod tss_ldt_flags {
    // First flags byte.
    pub const DPL           : u16 = 3 << 13; // 0b11 << 13
    pub const PRESENT       : u16 = 1 << 15;

    // Second flags byte.
    pub const LIMIT         : u16 = 0xF;
    pub const AVAILABLE     : u16 = 1 << 4;
    pub const GRANULARITY   : u16 = 1 << 7;
}

pub mod call_flags {
    pub const DPL           : u16 = 3 << 13; // 0b11 << 13
    pub const PRESENT       : u16 = 1 << 15;
}
