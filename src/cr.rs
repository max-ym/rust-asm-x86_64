/// Trait means that structure represents a register in a processor.
/// Changes are not immediately commited to the real register. This
/// trait defines functions that allow to read and store the data
/// to the real register in the CPU.
pub trait Reg : Sized {

    /// Read the register and create the struct to represent the value.
    unsafe fn read() -> Self;

    /// Save the changes to real register.
    unsafe fn save(&self);

    /// Re-read the real register and overwrite current data in the structure.
    unsafe fn reset(&mut self) {
        *self = Self::read();
    }
}

#[repr(packed)]
#[derive(Clone, Copy)]
/// Control Register 3.
pub struct Cr3 {
    data    : u64,
}

/// Control Register 4.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Cr4 {
    data    : u64,
}

impl Reg for Cr3 {

    unsafe fn read() -> Self {
        let data: u64;
        asm!(
            "mov    rax, cr3"
        :   "={rax}" (data)
        ::: "intel"
        );

        Cr3 {
            data: data
        }
    }

    unsafe fn save(&self) {
        asm!(
            "mov    cr3, rax"
        ::  "{rax}" (self.data)
        ::  "intel"
        );
    }

    unsafe fn reset(&mut self) {
        *self = Self::read();
    }
}

impl Cr3 {

    /// Page-level write-through.
    pub fn pwt(&self) -> bool {
        self.data & (1 << 3) != 0
    }

    /// Page-level cache disable.
    pub fn pcd(&self) -> bool {
        self.data & (1 << 4) != 0
    }

    pub unsafe fn set_pwt(&mut self, val: bool) {
        let a = self.data & !(1 << 3);
        self.data = if val {
            a | (1 << 3)
        } else {
            a
        };
    }

    pub unsafe fn set_pcd(&mut self, val: bool) {
        let a = self.data & !(1 << 4);
        self.data = if val {
            a | (1 << 4)
        } else {
            a
        };
    }

    /// Address of PML4 (P4)
    pub fn addr(&self) -> usize {
        (self.data & !0x7FF) as usize
    }

    pub unsafe fn set_addr(&mut self, addr: usize) {
        let a = self.data & 0x7FF;
        self.data = a + addr as u64;
    }
}

impl Reg for Cr4 {

    unsafe fn read() -> Self {
        let data: u64;
        asm!(
            "mov    rax, cr4"
        :   "={rax}" (data)
        ::: "intel"
        );

        Cr4 { data }
    }

    unsafe fn save(&self) {
        asm!(
            "mov    cr4, rax"
        ::  "{rax}" (self.data)
        ::  "intel"
        );
    }
}

macro_rules! impl_cr4_fn {
    ($cons:ident, $get:ident, $set:ident, $unset:ident, $docs:expr) => (
        #[doc=$docs]
        pub fn $get(&self) -> bool {
            self.data & Self::$cons != 0
        }

        #[doc=$docs]
        pub fn $set(&mut self) {
            self.data |= Self::$cons;
        }

        #[doc=$docs]
        pub fn $unset(&mut self) {
            self.data &= !Self::$cons;
        }
    );

    ($cons:ident, $get:ident, $set:ident, $unset:ident) => {
        impl_cr4_fn!($cons, $get, $set, $unset, "");
    }
}

impl Cr4 {

    const VME           : u64 = 1 << 0x00;
    const PVI           : u64 = 1 << 0x01;
    const TSD           : u64 = 1 << 0x02;
    const DE            : u64 = 1 << 0x03;
    const PSE           : u64 = 1 << 0x04;
    const PAE           : u64 = 1 << 0x05;
    const MCE           : u64 = 1 << 0x06;
    const PGE           : u64 = 1 << 0x07;
    const PCE           : u64 = 1 << 0x08;
    const OSFXSR        : u64 = 1 << 0x09;
    const OSXMMEXCPT    : u64 = 1 << 0x0A;
    const VMXE          : u64 = 1 << 0x0D;
    const SMXE          : u64 = 1 << 0x0E;
    const FSGSBASE      : u64 = 1 << 0x10;
    const PCIDE         : u64 = 1 << 0x11;
    const OSXSAVE       : u64 = 1 << 0x12;
    const SMEP          : u64 = 1 << 0x14;
    const SMAP          : u64 = 1 << 0x15;
    const PKE           : u64 = 1 << 0x16;

    impl_cr4_fn!(
            VME                 , vme                   ,
            enable_vme          , disable_vme           );
    impl_cr4_fn!(
            PVI                 , pvi                   ,
            enable_pvi          , disable_pvi           );
    // TODO
    impl_cr4_fn!(
            OSXSAVE             , osxsave               ,
            enable_osxsave      , disable_osxsave       );
}
