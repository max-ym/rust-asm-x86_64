use super::{Entry, EntryHandle, EntryVariant};

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

pub struct PEntryHandle<'a> {
    data_ref: &'a u64
}

pub enum PEntryVariant {
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
            data : 1 << 7, // Turn on PS
        }
    }
}

impl Default for P2ERef {

    fn default() -> Self {
        P2ERef {
            data : 0 << 7, // PS is off
        }
    }
}

impl EntryVariant for PEntryVariant {
}

impl<'a> EntryHandle<'a> for PEntryHandle<'a> {

    type Variant = PEntryVariant;

    fn variant(&self) -> Self::Variant {
        unimplemented!()
    }
}
