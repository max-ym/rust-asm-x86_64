use super::msr::ApicBase;
use super::cpuid;

/// Local APIC handle.
pub struct LocalApic {
    apic_base_msr   : ApicBase,
}

/// List of all local APIC registers and their addresses.
#[repr(usize)]
#[derive(PartialEq, Clone, Copy)]
pub enum LocalApicReg {
    Id                      = 0x020, // RW (Nehalem RO)
    Version                 = 0x030, // RO

    TaskPriority            = 0x080, // RW
    ArbitrationPriority     = 0x090, // RO, 1
    ProcessorPriority       = 0x0A0, // RO
    Eoi                     = 0x0B0, // WO
    RemoteRead              = 0x0C0, // RO, 1
    LogicalDestination      = 0x0D0, // RW
    DestinationFormat       = 0x0E0, // RW
    SpuriousInterruptVector = 0x0F0, // RW
    Isr0                    = 0x100, // RO
    Isr1                    = 0x110, // RO
    Isr2                    = 0x120, // RO
    Isr3                    = 0x130, // RO
    Isr4                    = 0x140, // RO
    Isr5                    = 0x150, // RO
    Isr6                    = 0x160, // RO
    Isr7                    = 0x170, // RO
    Tmr0                    = 0x180, // RO
    Tmr1                    = 0x190, // RO
    Tmr2                    = 0x1A0, // RO
    Tmr3                    = 0x1B0, // RO
    Tmr4                    = 0x1C0, // RO
    Tmr5                    = 0x1D0, // RO
    Tmr6                    = 0x1E0, // RO
    Tmr7                    = 0x1F0, // RO
    Irr0                    = 0x200, // RO
    Irr1                    = 0x210, // RO
    Irr2                    = 0x220, // RO
    Irr3                    = 0x230, // RO
    Irr4                    = 0x240, // RO
    Irr5                    = 0x250, // RO
    Irr6                    = 0x260, // RO
    Irr7                    = 0x270, // RO
    ErrorStatus             = 0x280, // RO

    LvtCmci                 = 0x2F0, // RW
    InterruptCommand0       = 0x300, // RW
    InterruptCommand1       = 0x310, // RW
    LvtTimer                = 0x320, // RW
    LvtThermalSensor        = 0x330, // RW, 2
    LvtPerformanceCounters  = 0x340, // RW, 3
    LvtLint0                = 0x350, // RW
    LvtLint1                = 0x360, // RW
    LvtError                = 0x370, // RW
    InitialCount            = 0x380, // RW
    CurrentCount            = 0x390, // RO

    DivideConfiguration     = 0x3E0, // RW

    // 1  - Not supported on Pentium 4 and Xeon.
    //
    // 2  - Introduced in Pentium 4 and Xeon. This APIC registers and its
    //      associated functions are implementation-dependent and may not be
    //      present in future IA-32 or Intel 64 processors.
    //
    // 3  - Introduced in Pentium Pro. This APIC register and its
    //      associated function are implementation-dependent and may not be
    //      present in future IA-32 or Intel 64 processors.
}

/// Value of DivideConfiguration register of APIC.
pub struct DivideConfiguration {
    reg     : u32,
}

/// Divide value that is set in DivideConfiguration register.
pub enum DivideValue {
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
}

impl LocalApicReg {

    /// Pointer to given local APIC register.
    pub fn ptr32(&self, apic: &LocalApic) -> *const u32 {
        let addr = *self as usize + apic.base_addr();
        addr as _
    }

    /// Pointer to mutable local APIC register.
    pub fn ptr32_mut(&self, apic: &mut LocalApic) -> *mut u32 {
        self.ptr32(apic) as _
    }

    pub fn ptr64(&self, apic: &LocalApic) -> *const u64 {
        self.ptr32(apic) as _
    }

    pub fn ptr64_mut(&self, apic: &mut LocalApic) -> *mut u64 {
        self.ptr32(apic) as _
    }

    pub fn ptr128(&self, apic: &LocalApic) -> *const (u64, u64) {
        self.ptr32(apic) as _
    }

    pub fn ptr128_mut(&self, apic: &mut LocalApic) -> *mut (u64, u64) {
        self.ptr32(apic) as _
    }
}

// Macro to create basic getter functions for local APIC registers.
macro_rules! ro {
    ($x:tt, $y:tt) => {
        pub fn $y(&self) -> u32 {
            Self::val(LocalApicReg::$x)
        }
    };
}

// Macro to create basic setter functions for local APIC registers.
macro_rules! wo {
    ($x:tt, $y:tt) => {
        pub fn $y(&self, val: u32) {
            Self::sval(LocalApicReg::$x, val)
        }
    };
}

impl LocalApic {

    /// Get Local APIC reference if APIC is actually available.
    /// To determine the presence of APIC, CPUID instruction is used.
    pub fn new() -> Option<LocalApic> {
        if Self::local_apic_is_present() {
            unsafe {
                Some(Self::unsafe_new())
            }
        } else {
            None
        }
    }

    /// Get Local APIC reference.
    ///
    /// # Safety
    /// Does not check whether Local APIC exists
    /// so caller must be sure this operation is valid.
    pub unsafe fn unsafe_new() -> LocalApic {
        LocalApic { apic_base_msr : ApicBase::read() }
    }

    /// Check if local APIC is present in given system. Function
    /// uses CPUID to check feature availability. Because of that
    /// calls to this function are slow.
    pub fn local_apic_is_present() -> bool {
        cpuid::Features::get().local_apic_is_present()
    }

    /// Base address of local APIC registers mapped to RAM.
    pub fn base_addr(&self) -> usize {
        self.apic_base_msr.apic_base() as _
    }

    /// Re-map local APIC registers to given new address.
    pub unsafe fn set_base_addr(&mut self, base: usize) {
        self.apic_base_msr.set_apic_base(base as _);
        self.apic_base_msr.write();
    }

    /// Get divide configuration value.
    pub fn divide_configuration(&self) -> &DivideConfiguration {
        use self::LocalApicReg::DivideConfiguration as Offset;
        let addr = self.base_addr() + Offset as usize;
        unsafe { &*(addr as *const DivideConfiguration) }
    }

    /// Get divide configuration value.
    pub fn divide_configuration_mut(&mut self) -> &mut DivideConfiguration {
        unsafe { &mut *(self.divide_configuration() as *const _ as *mut _) }
    }
}

impl DivideConfiguration {

    /// Set register specified value.
    unsafe fn set_reg(&mut self, val: u32) {
        self.reg = val;
    }

    /// Set specified divide value.
    pub fn set(&mut self, div: DivideValue) {
        use self::DivideValue::*;
        unsafe {
            self.set_reg(match div {
                Div2    => 0b0000,
                Div4    => 0b0001,
                Div8    => 0b0010,
                Div16   => 0b0011,
                Div32   => 0b1000,
                Div64   => 0b1001,
                Div128  => 0b1010,
                Div1    => 0b1011,
            });
        }
    }

    /// Get value of this register.
    pub fn get(&self) -> DivideValue {
        use self::DivideValue::*;
        match self.reg {
            0b0000 => Div2  ,
            0b0001 => Div4  ,
            0b0010 => Div8  ,
            0b0011 => Div16 ,
            0b1000 => Div32 ,
            0b1001 => Div64 ,
            0b1010 => Div128,
            0b1011 => Div1  ,
            _      => unreachable!()
        }
    }
}
