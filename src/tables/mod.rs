/// Module implementing Global Descriptor Table.
pub mod gdt;

/// Module implementing Interrupt Descriptor Table.
pub mod idt;

/// Module related to memory paging tables.
pub mod paging;

/// Entry from EntryTable.
pub trait Entry: Sized {

    /// Get size in bytes of an entry type.
    fn size() -> usize {
        ::core::mem::size_of::<Self>()
    }
}

/// A handle for entry type. Some tables (like GDT) may contain entries of
/// different sizes. Other entries with same size may have different types
/// thus different fields and methods to work with them. Entry handle
/// for a single position in the entry table provides functionality
/// to define actual type of entry and its appropriate representation.
pub trait EntryHandle<'a> {

    /// A type of entry variants that this handle can return.
    type Variant: EntryVariant;

    fn variant(&self) -> Self::Variant;
}

/// A representation variant of some table entry. Is returned by
/// entry handle to represent specific entry type with its own fields
/// and methods.
pub trait EntryVariant {
}

/// Table of entries. Like GDT, IDT, paging tables.
pub trait Table<'a> {

    /// A type of handle that will be returned when accessing table field.
    type Handle: EntryHandle<'a>;

    /// Get entry handle by entry index in the table. None is returned if
    /// index breaks table limit.
    fn entry_handle(&self, index: u16) -> Option<Self::Handle>;

    /// Get limit of entry table. Limit is presented in bytes count.
    fn limit(&self) -> u16;

    /// Get entry count.
    /// For example, if one element has size of 8 bytes and limit is
    /// set to 16 then this function will return 2. If limit is set to
    /// value 15, then this function will return 1. If no elements can
    /// be stored in the table, 0 is returned.
    fn entry_count(&self) -> u16 {
        // add 1 because limit = 0 means one valid byte, limit = 1 means two...
        (self.limit() + 1) / Self::limit_step()
    }

    /// Check if given index breaks the limit of entry table.
    /// If so, there is no entry with given index in the table.
    fn limit_broken_by(&self, index: u16) -> bool {
        self.entry_count() >= index
    }

    /// Get address of the table.
    fn addr(&self) -> u64;

    /// An amount of bytes that is skipped when index is incremented by 1.
    /// This equals to the smallest entry size that can be stored in the
    /// table.
    fn limit_step() -> u16;

    /// Convert element index to minimal limit value of the handle that
    /// must be set so that this element could be accessed.
    fn limit_from_index(index: u16) -> u16 {
        (index + 1) * Self::limit_step() - 1
    }
}

/// Descriptor Table Register Value.
pub trait RegValue<'a>: Sized {

    type HandleType: Table<'a>;

    /// Write current value to appropriate DTR.
    unsafe fn write(&self);

    /// Read current value from appropriate DTR.
    fn read(&mut self);

    /// Create DtrValue struct from current value in DTR.
    fn new_from_reg() -> Self;

    /// Create new value with given base address and limit.
    fn new(addr: u64, limit: u16) -> Self;

    /// Get base address of DT.
    fn addr(&self) -> u64;

    /// Get limit of DT.
    fn limit(&self) -> u16;

    /// Set address of DT.
    unsafe fn set_addr(&mut self, addr: u64);

    /// Set limit of DT.
    unsafe fn set_limit(&mut self, limit: u16);

    /// Consume DTR value and get DT handle.
    fn into_table(self) -> Self::HandleType;

    /// Consume DT and get a DTR value that can be stored to phisical register.
    fn from_table(table: Self::HandleType) -> Self {
        Self::new(table.addr(), table.limit())
    }
}

/// Descriptor Privilege Level. Used in GDT and IDT.
#[repr(u32)]
pub enum Dpl {
    Dpl0 = 0,
    Dpl1 = 1,
    Dpl2 = 2,
    Dpl3 = 3,
}

impl Dpl {

    /// Convert number from 0 to 3 to corresponding DPL level.
    pub fn from_num(i: u32) -> Option<Self> {
        use self::Dpl::*;
        match i {
            0 => Some(Dpl0),
            1 => Some(Dpl1),
            2 => Some(Dpl2),
            3 => Some(Dpl3),
            _ => None
        }
    }
}

/// IA-32e mode descriptor type.
#[repr(u16)]
#[derive(PartialEq)]
pub enum DescriptorType {
    Ldt             = 0b0010,
    TssAvailable    = 0b1001,
    TssBusy         = 0b1011,
    CallGate        = 0b1100,
    InterruptGate   = 0b1110,
    TrapGate        = 0b1111,

    Reserved
}

impl From<u16> for DescriptorType {

    fn from(v: u16) -> Self {
        use self::DescriptorType::*;

        if v == Ldt             as _ { Ldt              } else
        if v == TssAvailable    as _ { TssAvailable     } else
        if v == TssBusy         as _ { TssBusy          } else
        if v == CallGate        as _ { CallGate         } else
        if v == InterruptGate   as _ { InterruptGate    } else
        if v == TrapGate        as _ { TrapGate         } else
        { Reserved }
    }
}

impl DescriptorType {

    /// Get the type of the descriptor from raw first 8 bytes.
    pub fn type_field_from_raw64(data: u64) -> u16 {
        ((data >> (8 + 32)) & 0xF) as _
    }

    /// Get the type of the descriptor from the flags field.
    pub fn type_field_from_flags(flags: u16) -> u16 {
        (flags >> 8) & 0xF
    }

    /// Convert number to DescriptorType without checking if the value
    /// is a valid enum variant.
    pub unsafe fn fast_from(v: u16) -> Self {
        ::core::mem::transmute(v)
    }
}

/// Descriptor Table entry limit field trait.
///
/// This trait indicates that table limit can be modified and provides
/// functions to change limit value.
pub trait DtLimit<'a>: Table<'a> {

    /// Set limit to given value. Function does not check if given limit
    /// is of a valid value.
    unsafe fn set_limit(&mut self, limit: u16);

    /// Set entry count of entry table. This count is converted
    /// to apropriate limit value and is set in the handle. This
    /// function does not check if element count does not exceed
    /// valid value.
    unsafe fn set_limit_by_entry_count(&mut self, count: u16) {
        self.set_limit(Self::limit_from_index(count));
    }
}
