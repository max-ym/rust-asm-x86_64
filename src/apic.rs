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

/// Version register local APIC type and version number.
pub enum VersionNumber {
    Discrete    (u8),
    Integrated  (u8),
}

/// MDA model used in Dfr.
#[repr(u8)]
pub enum MdaModel {
    Flat        = 0b1111,
    Cluster     = 0b0000,
}

/// Delivery status of LVT interrupts.
pub enum DeliveryStatus {
    Idle,
    SendPending,
}

/// ICR level field.
#[derive(Clone, Copy)]
pub enum IcrLevel {
    Assert,
    Deassert,
}

/// IPI destination mode.
#[derive(Clone, Copy)]
pub enum DestinationMode {
    Physical,
    Logical,
}

/// Destination shorthand of ICR.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum DestinationShorthand {

    /// Destination is set in destination field.
    NoShorthand         = 0b00,
    SelfDestination     = 0b01,
    AllIncludingSelf    = 0b10,
    AllExcludingSelf    = 0b11,
}

/// Delivery mode of LVT interrupt.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum DeliveryMode {
    Fixed   = 0b000,
    Smi     = 0b010,
    Nmi     = 0b100,
    ExtInt  = 0b111,
    Init    = 0b101,
}

/// Interrupt input pin polarity.
pub enum PinPolarity {
    ActiveHigh,
    ActiveLow,
}

/// LVT interrupt trigger mode.
pub enum TriggerMode {
    EdgeSensitive,
    LevelSensitive,
}

/// Arbitration priority class or sub-class value.
pub struct PriorityClass {
    val : u8,
}

/// Version register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Version {
    reg     : u32,
}

/// Task priority register.
#[repr(packed)]
pub struct Tpr {
    class   : u8,

    _resv0  : u8,
    _resv1  : u16,
}

/// Arbitration priority register.
#[repr(packed)]
pub struct Apr {
    class   : u8,

    _resv0  : u8,
    _resv1  : u16,
}

/// Value of DivideConfiguration register of APIC.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct DivideConfiguration {
    reg     : u32,
}

/// End Of Interrupt register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Eoi {
    reg     : u32,
}

/// Logical destination register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Ldr {
    reg     : u32,
}

/// Destination format register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Dfr {
    _resv0  : u16,
    _resv1  : u8,

    model   : u8,
}

/// Spurious Interrupt register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct SpuriousInterrupt {
    reg     : u32,
}

/// LVT CMCI register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LvtCmci {
    reg     : u32,
}

/// Interrupt command register 0.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Icr0 {
    reg     : u32
}

/// Interrupt command register 1.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Icr1 {
    reg     : u32
}

/// Interrupt command register interface. Takes care of writing values
/// into the APIC registers in right order and time.
pub struct Icr<'a> {
    icr0: &'a mut Icr0,
    icr1: &'a mut Icr1,

    icr0_pending: Icr0,
    icr1_pending: Icr1,
}

/// Value of Local Vector Table Timer register of APIC.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LvtTimer {
    reg     : u32,
}

/// LVT Thermal Sensor register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LvtThermalSensor {
    reg     : u32,
}

/// LVT performance counters register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LvtPerformanceCounters {
    reg     : u32,
}

/// LVT local interrupt pin 0 register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LvtLint0 {
    reg     : u32,
}

/// LVT local interrupt pin 1 register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LvtLint1 {
    reg     : u32,
}

/// LVT error interrupt register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LvtError {
    reg     : u32,
}

/// LVT Timer mode.
#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
pub enum LvtTimerMode {
    OneShot     = 0b00 << 17,
    Periodic    = 0b01 << 17,
    TscDeadline = 0b10 << 17,
}

/// Mask that can be used to clear out all bits except timer mode
/// in LVT Timer register.
const LVT_TIMER_MODE_MASK: u32 = 0b11 << 17;

/// Value of current timer count register of APIC.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TimerCurrentCount {
    reg     : u32,
}

/// Value of initial timer counter register of APIC.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TimerInitialCount {
    reg     : u32,
}

/// Divide value that is set in DivideConfiguration register.
#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
pub enum DivideValue {
    Div1    = 0b1011,
    Div2    = 0b0000,
    Div4    = 0b0001,
    Div8    = 0b0010,
    Div16   = 0b0011,
    Div32   = 0b1000,
    Div64   = 0b1001,
    Div128  = 0b1010,
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

impl VersionNumber {

    /// Create from given version number.
    pub fn from_number(num: u8) -> Self {
        if num <= 0xF {
            VersionNumber::Discrete(num)
        } else {
            VersionNumber::Integrated(num)
        }
    }
}

impl From<u8> for MdaModel {

    fn from(val: u8) -> MdaModel {
        unsafe { ::core::mem::transmute(val) }
    }
}

impl From<u32> for DestinationShorthand {

    fn from(v: u32) -> DestinationShorthand {
        unsafe { ::core::mem::transmute(v) }
    }
}

impl From<u32> for DeliveryMode {

    fn from(v: u32) -> DeliveryMode {
        unsafe { ::core::mem::transmute(v) }
    }
}

macro_rules! lvt_entry_impl_base {
    () => {

        /// LVT vector.
        pub fn vector(&self) -> u8 {
            (self.reg & 0xF) as u8
        }

        /// Set LVT vector.
        pub fn set_vector(&mut self, vec: u8) {
            self.reg = self.reg & !0xF | (vec as u32);
        }

        /// Delivery status.
        pub fn delivery_status(&self) -> DeliveryStatus {
            use self::DeliveryStatus::*;

            if self.reg & (1 << 12) != 0 {
                SendPending
            } else {
                Idle
            }
        }

        /// Whether interrupt is masked or not.
        pub fn masked(&self) -> bool {
            self.reg & (1 << 16) != 0
        }

        /// Mask this interrupt.
        pub fn mask(&mut self) {
            self.reg |= 1 << 16;
        }

        /// Unmask this interrupt.
        pub fn unmask(&mut self) {
            self.reg &= !(1 << 16);
        }
    };
}

macro_rules! lvt_entry_impl_delivery {
    () => {
        /// Delivery mode.
        pub fn delivery_mode(&self) -> DeliveryMode {
            DeliveryMode::from((self.reg >> 8) & 0b111)
        }

        /// Set delivery mode without checking other fields for valid values.
        pub unsafe fn only_set_delivery_mode(&mut self, mode: DeliveryMode) {
            let val = mode as u32;
            let mask = val << 8;
            self.reg = self.reg & !(0b111 << 8) | mask;
        }
    }
}

macro_rules! lvt_entry_impl_lint {
    () => {

        /// Trigger mode.
        pub fn trigger_mode(&self) -> TriggerMode {
            use self::TriggerMode::*;

            if self.reg & (1 << 15) != 0 {
                LevelSensitive
            } else {
                EdgeSensitive
            }
        }

        /// Whether remote IRR flag is on.
        pub fn remote_irr(&self) -> bool {
            self.reg & (1 << 14) != 0
        }

        /// Interrupt input pin polarity.
        pub fn input_polarity(&self) -> PinPolarity {
            use self::PinPolarity::*;

            if self.reg & (1 << 13) != 0 {
                ActiveLow
            } else {
                ActiveHigh
            }
        }

        /// Set interrupt input pin polarity.
        pub fn set_input_polarity(&mut self, pp: PinPolarity) {
            use self::PinPolarity::*;

            match pp {
                ActiveHigh  => self.reg &= !(1 << 13),
                ActiveLow   => self.reg |=   1 << 13
            }
        }
    }
}

macro_rules! lapic_reg_ref_impl {
    ($n:ident, $nm:ident, $ty:tt, $doc:expr) => {
        lapic_reg_ref_impl!($ty, $n, $nm, $ty, $doc);
    };

    ($enu:ident, $n:ident, $nm:ident, $ty:tt, $doc:expr) => {
        #[doc=$doc]
        pub fn $n(&self) -> &$ty {
            let ptr = LocalApicReg::$enu.ptr32(self);
            unsafe { &*(ptr as *const _) }
        }

        #[doc=$doc]
        pub fn $nm(&mut self) -> &mut $ty {
            let ptr = LocalApicReg::$enu.ptr32_mut(self);
            unsafe { &mut *(ptr as *mut _) }
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

    /// Version register.
    pub fn version(&self) -> &Version {
        let ptr = LocalApicReg::Version.ptr32(self);
        unsafe { &*(ptr as *const _) }
    }

    /// EOI register.
    pub fn eoi_mut(&mut self) -> &mut Eoi {
        let ptr = LocalApicReg::Eoi.ptr32_mut(self);
        unsafe { &mut *(ptr as *mut _) }
    }

    lapic_reg_ref_impl!(SpuriousInterruptVector,
            spurious_interrupt, spurious_interrupt_mut,
            SpuriousInterrupt, "Spurious interrupt vector register.");

    lapic_reg_ref_impl!(lvt_cmci, lvt_cmci_mut,
            LvtCmci, "LVT CMCI register.");

    lapic_reg_ref_impl!(lvt_timer, lvt_timer_mut,
            LvtTimer, "LVT timer register.");

    lapic_reg_ref_impl!(lvt_thermal_sensor, lvt_thermal_sensor_mut,
            LvtThermalSensor, "LVT thermal sensor register.");

    lapic_reg_ref_impl!(InitialCount,
            initial_count, initial_count_mut,
            TimerInitialCount, "Timer initial count register.");

    lapic_reg_ref_impl!(CurrentCount,
            current_count, current_count_mut,
            TimerCurrentCount, "Timer current count register.");

    lapic_reg_ref_impl!(divide_configuration, divide_configuration_mut,
            DivideConfiguration, "Divide configuration value.");

    /// ICR registers interface.
    pub fn icr_interface(&mut self) -> Icr {
        let icr0_ptr = LocalApicReg::InterruptCommand0.ptr32_mut(self);
        let icr1_ptr = LocalApicReg::InterruptCommand1.ptr32_mut(self);

        let icr0 = unsafe { &mut *(icr0_ptr as *mut Icr0) };
        let icr1 = unsafe { &mut *(icr1_ptr as *mut Icr1) };

        let icr0_val = icr0.clone();
        let icr1_val = icr1.clone();

        Icr {
            icr0 : icr0,
            icr1 : icr1,

            icr0_pending : icr0_val,
            icr1_pending : icr1_val,
        }
    }

    lapic_reg_ref_impl!(LogicalDestination, ldr, ldr_mut,
            Ldr, "Logical destination register.");

    lapic_reg_ref_impl!(DestinationFormat, dfr, dfr_mut,
            Dfr, "Destination format register.");

    lapic_reg_ref_impl!(ArbitrationPriority, apr, apr_mut,
            Apr, "Arbitration priority register.");

    lapic_reg_ref_impl!(TaskPriority, tpr, tpr_mut,
            Tpr, "Task priority register.");
}

impl PriorityClass {

    /// Try converting given class value to instance of this class.
    /// This can be possible only for number between 0 and 15 inclusive as
    /// only these values represent class or sub-class field (which is 4 bits
    /// in size - 16 different values can be stored).
    pub fn try_new(val: u8) -> Option<Self> {
        if val > 0x0F {
            None
        } else {
            Some(PriorityClass { val : val })
        }
    }

    /// Value of priority class.
    pub fn value(&self) -> u8 {
        self.val
    }
}

impl Into<u8> for PriorityClass {

    fn into(self) -> u8 {
        self.value()
    }
}

impl Version {

    /// Local APIC version number.
    pub fn version(&self) -> VersionNumber {
        VersionNumber::from_number((self.reg & 0xF) as _)
    }

    /// Max LVT entry count minus one.
    pub fn max_lvt_entry(&self) -> u8 {
        ((self.reg & 0x0F_00) >> 16) as u8
    }

    /// Check support of EOI broadcast suppression.
    pub fn eoi_broadcast_suppression(&self) -> bool {
        self.reg & 0x10_00 != 0
    }
}

impl Tpr {

    /// Class field.
    pub fn class_field(&self) -> u8 {
        self.class
    }

    /// Set class field.
    pub fn set_class_field(&mut self, class: u8) {
        self.class = class;
    }

    /// Arbitration priority sub-class.
    pub fn subclass(&self) -> PriorityClass {
        PriorityClass::try_new(self.class & 0x0F).unwrap()
    }

    pub fn set_subclass(&mut self, class: PriorityClass) {
        let mask: u8 = class.into();
        self.class = self.class & 0xF0 | mask;
    }

    /// Arbitration priority class.
    pub fn class(&self) -> PriorityClass {
        let val = self.class >> 4;
        PriorityClass::try_new(val).unwrap()
    }

    pub fn set_class(&mut self, class: PriorityClass) {
        let mask: u8 = class.into();
        let mask = mask << 4;
        self.class = self.class & 0x0F | mask;
    }
}

impl Apr {

    /// Class field.
    pub fn class_field(&self) -> u8 {
        self.class
    }

    /// Set class field.
    pub fn set_class_field(&mut self, class: u8) {
        self.class = class;
    }

    /// Arbitration priority sub-class.
    pub fn subclass(&self) -> PriorityClass {
        PriorityClass::try_new(self.class & 0x0F).unwrap()
    }

    pub fn set_subclass(&mut self, class: PriorityClass) {
        let mask: u8 = class.into();
        self.class = self.class & 0xF0 | mask;
    }

    /// Arbitration priority class.
    pub fn class(&self) -> PriorityClass {
        let val = self.class >> 4;
        PriorityClass::try_new(val).unwrap()
    }

    pub fn set_class(&mut self, class: PriorityClass) {
        let mask: u8 = class.into();
        let mask = mask << 4;
        self.class = self.class & 0x0F | mask;
    }
}

impl Eoi {

    /// Send End-Of-Interrupt signal.
    pub fn signal(&mut self) {
        self.reg = 0;
    }
}

impl Ldr {

    pub fn logical_apic_id(&self) -> u8 {
        (self.reg >> 24) as _
    }

    pub fn set_logical_apic_id(&mut self, val: u8) {
        self.reg = self.reg & 0x00FF_FFFF | ((val as u32) << 24);
    }
}

impl Dfr {

    pub fn model(&self) -> MdaModel {
        MdaModel::from(self.model)
    }

    pub fn set_model(&mut self, model: MdaModel) {
        self.model = model as _;
    }
}

impl SpuriousInterrupt {

    /// Spurious interrupt vector.
    pub fn vector(&self) -> u8 {
        (self.reg & 0xFF) as _
    }

    /// Set spurious interrupt vector.
    pub fn set_vector(&mut self, vec: u8) {
        self.reg = self.reg & !0xFF | (vec as u32);
    }

    /// EOI broadcast suppression.
    pub fn eoi_broadcast_suppression(&self) -> bool {
        self.reg & 0b1_0000_0000_0000 != 0
    }

    /// Enable EOI broadcast suppression.
    ///
    /// # Safety
    /// Not supported on some CPUs.
    pub unsafe fn enable_eoi_broadcast_suppression(&mut self) {
        self.reg |= 0b1_0000_0000_0000;
    }

    pub fn disable_eoi_broadcast_suppression(&mut self) {
        self.reg &= !0b1_0000_0000_0000;
    }

    pub fn focus_processor_checking(&self) -> bool {
        // NOTE if flag is zero then checking is enabled!
        self.reg & 0b0010_0000_0000 == 0
    }

    pub fn enable_focus_processor_checking(&mut self) {
        self.reg &= !0b0010_0000_0000;
    }

    /// Disable focus processor checking.
    ///
    /// # Safety
    /// Not supported on Pentium 4 and Xeon processors.
    pub unsafe fn disable_focus_processor_checking(&mut self) {
        self.reg |= 0b0010_0000_0000;
    }

    pub fn is_apic_software_enabled(&self) -> bool {
        self.reg & 0b0001_0000_0000 != 0
    }

    pub fn software_enable_apic(&mut self) {
        self.reg |= 0b0001_0000_0000;
    }

    pub fn software_disable_apic(&mut self) {
        self.reg &= !0b0001_0000_0000;
    }
}

impl LvtCmci {
    lvt_entry_impl_base!();
    lvt_entry_impl_delivery!();
}

impl Icr0 {

    /// Interrupt vector.
    pub fn vector(&self) -> u8 {
        (self.reg & 0xF) as u8
    }

    /// Set new interrupt vector.
    pub fn set_vector(&mut self, vec: u8) {
        self.reg = self.reg & !0xF | (vec as u32);
    }

    /// Interrupt delivery mode.
    pub fn delivery_mode(&self) -> DeliveryMode {
        let val = (self.reg >> 8) & 0b111;
        DeliveryMode::from(val)
    }

    /// Set delivery mode.
    pub fn set_delivery_mode(&mut self, mode: DeliveryMode) {
        let val = mode as u32;
        self.reg = self.reg & !(0b111 << 8) | (val << 8);
    }

    /// IPI destination mode.
    pub fn destination_mode(&self) -> DestinationMode {
        use self::DestinationMode::*;

        if self.reg & (1 << 11) != 0 {
            Logical
        } else {
            Physical
        }
    }

    /// Set IPI destination mode.
    pub fn set_destination_mode(&mut self, mode: DestinationMode) {
        use self::DestinationMode::*;

        match mode {
            Logical     => self.reg |=   1 << 11,
            Physical    => self.reg &= !(1 << 11)
        }
    }

    /// Delivery status.
    pub fn delivery_status(&self) -> DeliveryStatus {
        use self::DeliveryStatus::*;

        if self.reg & (1 << 11) != 0 {
            SendPending
        } else {
            Idle
        }
    }

    pub fn level(&self) -> IcrLevel {
        use self::IcrLevel::*;

        if self.reg & (1 << 14) != 0 {
            Assert
        } else {
            Deassert
        }
    }

    pub fn set_level(&mut self, level: IcrLevel) {
        use self::IcrLevel::*;

        match level {
            Assert      => self.reg |=   1 << 14,
            Deassert    => self.reg &= !(1 << 14)
        }
    }

    pub fn trigger_mode(&self) -> TriggerMode {
        use self::TriggerMode::*;

        if self.reg & (1 << 15) != 0 {
            LevelSensitive
        } else {
            EdgeSensitive
        }
    }

    pub fn set_trigger_mode(&mut self, mode: TriggerMode) {
        use self::TriggerMode::*;

        match mode {
            EdgeSensitive   => self.reg &= !(1 << 15),
            LevelSensitive  => self.reg |=   1 << 15
        }
    }

    pub fn destination_shorthand(&self) -> DestinationShorthand {
        use self::DestinationShorthand::*;

        DestinationShorthand::from((self.reg >> 18) & 0b11)
    }

    pub fn set_destination_shorthand(&mut self, ds: DestinationShorthand) {
        let val = (ds as u32) << 18;
        self.reg = self.reg & (0b11 << 18) | val;
    }
}

impl Icr1 {

    pub fn destination(&self) -> u8 {
        (self.reg >> 24) as _
    }

    pub fn set_destination(&mut self, dest: u8) {
        self.reg = self.reg & (0xFF << 24) | ((dest as u32) << 24);
    }
}

impl<'a> Icr<'a> {

    pub fn vector(&self) -> u8 {
        self.icr0_pending.vector()
    }

    pub fn set_vector(&mut self, vec: u8) {
        self.icr0_pending.set_vector(vec)
    }

    pub fn delivery_mode(&self) -> DeliveryMode {
        self.icr0_pending.delivery_mode()
    }

    pub fn set_delivery_mode(&mut self, mode: DeliveryMode) {
        self.icr0_pending.set_delivery_mode(mode)
    }

    pub fn destination_mode(&self) -> DestinationMode {
        self.icr0_pending.destination_mode()
    }

    pub fn set_destination_mode(&mut self, mode: DestinationMode) {
        self.icr0_pending.set_destination_mode(mode)
    }

    pub fn delivery_status(&self) -> DeliveryStatus {
        // Current value, not pending one.
        self.icr0.delivery_status()
    }

    pub fn level(&self) -> IcrLevel {
        self.icr0_pending.level()
    }

    pub fn set_level(&mut self, level: IcrLevel) {
        self.icr0_pending.set_level(level)
    }

    pub fn trigger_mode(&self) -> TriggerMode {
        self.icr0_pending.trigger_mode()
    }
    pub fn set_trigger_mode(&mut self, mode: TriggerMode) {
        self.icr0_pending.set_trigger_mode(mode)
    }

    pub fn destination_shorthand(&self) -> DestinationMode {
        self.icr0_pending.destination_mode()
    }

    pub fn set_destination_shorthand(&mut self, ds: DestinationShorthand) {
        self.icr0_pending.set_destination_shorthand(ds)
    }

    pub fn destination(&self) -> u8 {
        self.icr1_pending.destination()
    }

    pub fn set_destination(&mut self, dest: u8) {
        self.icr1_pending.set_destination(dest)
    }

    /// Restore ICR0 value from registers.
    pub fn restore_icr0(&mut self) {
        self.icr0_pending = self.icr0.clone();
    }

    /// Restore ICR1 value from registers.
    pub fn restore_icr1(&mut self) {
        self.icr1_pending = self.icr1.clone();
    }

    /// Save pending values to real registers.
    pub fn apply(&mut self) {
        // Values must be stored in given order:
        *self.icr1 = self.icr1_pending;
        *self.icr0 = self.icr0_pending;
    }

    /// Save changes only from icr0.
    pub fn apply_icr0(&mut self) {
        *self.icr0 = self.icr0_pending;
    }
}

impl LvtTimer {
    lvt_entry_impl_base!();

    /// Current LVT Timer mode.
    pub fn mode(&self) -> LvtTimerMode {
        let val = self.reg & LVT_TIMER_MODE_MASK;
        unsafe { ::core::mem::transmute(val) }
    }

    /// Set LVT Timer mode.
    ///
    /// # Safety
    /// Caller must ensure that given mode is supported.
    /// TSC Deadline mode is not supported on some CPUs.
    pub unsafe fn set_mode(&mut self, mode: LvtTimerMode) {
        self.reg = self.reg & !LVT_TIMER_MODE_MASK | mode as u32;
    }
}

impl LvtThermalSensor {
    lvt_entry_impl_base!();
    lvt_entry_impl_delivery!();
}

impl LvtPerformanceCounters {
    lvt_entry_impl_base!();
    lvt_entry_impl_delivery!();
}

impl LvtLint0 {
    lvt_entry_impl_base!();
    lvt_entry_impl_delivery!();
    lvt_entry_impl_lint!();

    /// Set trigger mode. Note that some delivery modes ignore this value
    /// and use their default trigger mode.
    pub fn set_trigger_mode(&mut self, mode: TriggerMode) {
        use self::TriggerMode::*;

        match mode {
            EdgeSensitive   => self.reg &= !(1 << 15),
            LevelSensitive  => self.reg |=   1 << 15
        }
    }
}

impl LvtLint1 {
    lvt_entry_impl_base!();
    lvt_entry_impl_delivery!();
    lvt_entry_impl_lint!();
}

impl LvtError {
    lvt_entry_impl_base!();
}

impl TimerCurrentCount {

    /// Get current timer value.
    pub fn value(&self) -> u32 {
        self.reg
    }
}

impl TimerInitialCount {

    /// Get initial timer value.
    pub fn value(&self) -> u32 {
        self.reg
    }

    /// Set new initial timer value.
    pub fn set(&mut self, value: u32) {
        self.reg = value;
    }

    /// Stop the timer by setting zero value.
    pub fn stop_timer(&mut self) {
        // This stops the timer according to Intel manual.
        self.set(0);
    }
}

impl DivideConfiguration {

    /// Set register specified value.
    unsafe fn set_reg(&mut self, val: u32) {
        self.reg = val;
    }

    /// Set specified divide value.
    pub fn set(&mut self, div: DivideValue) {
        unsafe { self.set_reg(div as _); }
    }

    /// Set divide factor 1.
    pub fn set_1(&mut self) {
        self.set(DivideValue::Div1);
    }

    /// Set divide factor 2.
    pub fn set_2(&mut self) {
        self.set(DivideValue::Div2);
    }

    /// Set divide factor 4.
    pub fn set_4(&mut self) {
        self.set(DivideValue::Div4);
    }

    /// Set divide factor 8.
    pub fn set_8(&mut self) {
        self.set(DivideValue::Div8);
    }

    /// Set divide factor 16.
    pub fn set_16(&mut self) {
        self.set(DivideValue::Div16);
    }

    /// Set divide factor 32.
    pub fn set_32(&mut self) {
        self.set(DivideValue::Div32);
    }

    /// Set divide factor 64.
    pub fn set_64(&mut self) {
        self.set(DivideValue::Div64);
    }

    /// Set divide factor 128.
    pub fn set_128(&mut self) {
        self.set(DivideValue::Div128);
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
