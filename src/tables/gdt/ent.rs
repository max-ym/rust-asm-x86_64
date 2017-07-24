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

/// GDT entry variant.
pub enum GdtVariant<'a> {
    Null    (&'a NullDescriptor     ),
    Call    (&'a CallGateDescriptor ),
    Tss     (&'a TssDescriptor      ),
    Ldt     (&'a LdtDescriptor      ),

    Unknown
}

impl<'a> EntryVariant for GdtVariant<'a> {
}

impl CallGateDescriptor {

    pub fn flags(&self) -> u16 {
        self.flags
    }

    pub unsafe fn set_flags(&mut self, flags: u16) {
        self.flags = flags;
    }

    pub fn masked_flags(&self, mask: u16) -> u16 {
        self.flags & mask
    }

    pub fn unmasked_flags(&self, mask: u16) -> u16 {
        self.flags & !mask
    }
}

macro_rules! impl_tss_ldt {
    ($name:ident) => (
        impl $name {

            pub fn flags(&self) -> (u16, u8) {
                (self.flags0, self.flags1)
            }

            pub unsafe fn set_flags0(&mut self, flags: u16) {
                self.flags0 = flags;
            }

            pub unsafe fn set_flags1(&mut self, flags: u8) {
                self.flags1 = flags;
            }

            pub fn masked_flags0(&self, mask: u16) -> u16 {
                self.flags0 & mask
            }

            pub fn unmasked_flags0(&self, mask: u16) -> u16 {
                self.flags0 & !mask
            }

            pub fn masked_flags1(&self, mask: u8) -> u8 {
                self.flags1 & mask
            }

            pub fn unmasked_flags1(&self, mask: u8) -> u8 {
                self.flags1 & !mask
            }

            pub unsafe fn set_base(&mut self, base: u64) {
                self.base0 = ((base & 0x00000000_0000FFFF) << 0x00) as _;
                self.base1 = ((base & 0x00000000_00FF0000) << 0x10) as _;
                self.base2 = ((base & 0x00FFFFFF_FF000000) << 0x18) as _;
            }
        }
    );
}

impl_tss_ldt!(TssDescriptor);
impl_tss_ldt!(LdtDescriptor);
