
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

    /// Get entry handle by entry index in the table.
    fn entry_handle(&self, index: u16) -> Self::Handle;

    /// Get limit of entry table. Limit is presented in bytes count.
    fn limit(&self) -> u16;

    /// Get entry count.
    /// For example, if one element has size of 8 bytes and limit is
    /// set to 16 then this function will return 2. If limit is set to
    /// value 15, then this function will return 1. If no elements can
    /// be stored in the table, 0 is returned.
    fn entry_count(&self) -> u16;

    /// Check if given index breaks the limit of entry table.
    /// If so, there is no entry with given index in the table.
    fn limit_broken_by(&self, index: u16) -> bool {
        self.entry_count() >= index
    }

    /// Get address of the table.
    fn addr(&self) -> u64;
}
