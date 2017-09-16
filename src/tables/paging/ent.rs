use super::{Entry, EntryVariant};

/// Page Table entry. Page table level 1 entry. Maps 4KiB page.
#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct P1E {
    data: u64
}

/// Page Directory entry. Page table level 2 entry. Maps 2MiB page.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct P2EMap {
    data: u64
}

/// Page Directory entry. Page table level 2 entry. References P1 table.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct P2ERef {
    data: u64
}

/// Page Directory Pointer entry. Page table level 3 entry.
#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct P3E {
    data: u64
}

/// Page Map Level 4 entry. Page table level 4 entry.
#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct P4E {
    data: u64
}

impl Entry for P1E {
}

impl Entry for P2EMap {
}

impl Entry for P2ERef {
}

impl Entry for P3E {
}

impl Entry for P4E {
}

impl Default for P2EMap {

    fn default() -> Self {
        P2EMap {
            data : PageFlag::ps().into() // PS is on.
        }
    }
}

impl Default for P2ERef {

    fn default() -> Self {
        P2ERef {
            data : 0 // PS is off.
        }
    }
}

pub enum P1EVariant<'a> {
    P1E(&'a P1E)
}

pub enum P2EVariant<'a> {
    Map(&'a P2EMap),
    Ref(&'a P2ERef),
}

pub enum P3EVariant<'a> {
    P3E(&'a P3E)
}

pub enum P4EVariant<'a> {
    P4E(&'a P4E)
}

impl<'a> EntryVariant for P1EVariant<'a> {
}

impl<'a> EntryVariant for P2EVariant<'a> {
}

impl<'a> EntryVariant for P3EVariant<'a> {
}

impl<'a> EntryVariant for P4EVariant<'a> {
}

new_bitflags! {
    pub flags PageFlag: u64 {
        const present   = 1 << 0x00;
        const rw        = 1 << 0x01;
        const us        = 1 << 0x02;
        const pwt       = 1 << 0x03;
        const pcd       = 1 << 0x04;
        const accessed  = 1 << 0x05;
        const dirty     = 1 << 0x06;
        const pat       = 1 << 0x07;
        const ps        = 1 << 0x07;
        const global    = 1 << 0x08;
        const xd        = 1 << 0x3F;

        const p1addr    = 0x0007FFFFFFFFF800;
        const p2addrmap = 0x0007FFFFFFFFF800;
        const p2addrref = 0x0007FFFFFFFFF000;
        const p3addr    = 0x0007FFFFFFFFF800;
        const p4addr    = 0x0007FFFFFFFFF800;
    }
}

impl Into<u64> for PageFlag {

    fn into(self) -> u64 {
        unsafe { ::core::mem::transmute(self) }
    }
}

impl From<u64> for PageFlag {

    fn from(v: u64) -> Self {
        unsafe { ::core::mem::transmute(v) }
    }
}

macro_rules! _impl {
    ($name:tt) => (
        impl $name {

            /// Perform bitwise 'or' on entry.
            ///
            /// # Safety
            /// Because the passed data is uncontrolled even invalid values
            /// may be set.
            pub unsafe fn data_bitwise_or(&mut self, val: u64) {
                self.data |= val;
            }

            /// Perform bitwise 'or' on entry.
            ///
            /// # Safety
            /// Because the passed data is uncontrolled even invalid values
            /// may be set.
            pub unsafe fn data_or(&mut self, val: PageFlag) {
                self.data_bitwise_or(val.into())
            }

            /// Disable all the bits of the
            /// entry bitwise representation that are set in mask.
            /// Then perform bitwise 'or' with the given value. It is not
            /// checked if the value is lying inside the given mask.
            ///
            /// # Safety
            /// Because the passed data is uncontrolled even invalid values
            /// may be set.
            pub unsafe fn data_bitwise_replace(&mut self, mask: u64,
                    val: u64) {
                self.data_bitwise_clear(mask);
                self.data_bitwise_or(val);
            }

            /// Disable all the bits of the
            /// entry bitwise representation that are set in mask.
            /// Then perform bitwise 'or' with the given value. It is not
            /// checked if the value is lying inside the given mask.
            ///
            /// # Safety
            /// Because the passed data is uncontrolled even invalid values
            /// may be set.
            pub unsafe fn data_replace(&mut self, mask: PageFlag,
                    val: PageFlag) {
                self.data_bitwise_replace(mask.into(), val.into())
            }

            /// Ignore old value of the field and rewrite them with
            /// given flags.
            ///
            /// # Safety
            /// Because the passed data is uncontrolled even invalid values
            /// may be set.
            pub unsafe fn data_rewrite(&mut self, val: PageFlag) {
                self.data = val.into();
            }

            /// Disable all the bits from entry bitfield representation
            /// that are enabled in the given mask.
            ///
            /// # Safety
            /// Because the passed data is uncontrolled even invalid values
            /// may be set.
            pub unsafe fn data_bitwise_clear(&mut self, mask: u64) {
                self.data &= !mask;
            }

            /// Disable all the bits from entry bitfield representation
            /// that are enabled in the given mask.
            ///
            /// # Safety
            /// Because the passed data is uncontrolled even invalid values
            /// may be set.
            pub unsafe fn data_clear(&mut self, mask: PageFlag) {
                self.data_bitwise_clear(mask.into())
            }
        }
    );
}

_impl!(P1E);
_impl!(P2EMap);
_impl!(P2ERef);
_impl!(P3E);
_impl!(P4E);
