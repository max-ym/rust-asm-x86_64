#![allow(dead_code)]

/// Information stored by CPUID instruction in appropriate registers.
#[derive(Clone, Copy)]
pub struct Info {
    pub eax     : u32,
    pub ebx     : u32,
    pub ecx     : u32,
    pub edx     : u32,
}

/// Enum that stores all valid CPUID query codes.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum InfoType {
    VendorString    = 0x00,
    Features        = 0x01,
    Tlb             = 0x02,
    Serial          = 0x03,

    // Xsave           = 0x0D, // Sub-function needs to be specified too.

    IntelExtended       = 0x8000_0000,
    IntelFeatures       = 0x8000_0001,
    IntelBrandString    = 0x8000_0002,
    IntelBrandStringMore= 0x8000_0003,
    IntelBrandStringEnd = 0x8000_0004,
}

/// XSAVE information types. Is passed to CPUID in ECX register.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum XsaveInfoType {
    Subf0       = 0x00,
    Subf1       = 0x01,
}

impl Info {

    /// Run CPUID instruction to query information.
    #[inline(always)]
    pub fn get(info: InfoType) -> Self {
        Self::get_by_code(info as u32)
    }

    #[inline(always)]
    pub fn get_xsave(info: XsaveInfoType) -> Self {
        let xsave_cpuid_number = 0x0D;
        Self::get_by_code_ecx(xsave_cpuid_number, info as _)
    }

    #[inline(always)]
    pub fn get_by_code(request: u32) -> Self {
        let (a, b, c, d);

        unsafe { asm!(
            "cpuid"
            : "={eax}"(a), "={ebx}"(b), "={ecx}"(c), "={edx}"(d)
            : "{eax}"(request)
        ); }

        Info { eax:a, ebx:b, ecx:c, edx:d }
    }

    #[inline(always)]
    pub fn get_by_code_ecx(request: u32, subfunction: u32) -> Self {
        let (a, b, c, d);

        unsafe { asm!(
            "cpuid"
            : "={eax}"(a), "={ebx}"(b), "={ecx}"(c), "={edx}"(d)
            : "{eax}"(request), "{ecx}"(subfunction)
        ); }

        Info { eax:a, ebx:b, ecx:c, edx:d }
    }
}

macro_rules! derive_conversions {
    ($x:ident) => (
        #[derive(Clone, Copy)]
        pub struct $x {
            info    : Info
        }

        impl From<Info> for $x {

            fn from(i: Info) -> Self {
                $x { info:i }
            }
        }

        impl Into<Info> for $x {

            fn into(self) -> Info {
                self.info
            }
        }
    );
}

macro_rules! derive_info {
    ($x:ident) => (
        derive_conversions!($x);

        impl $x {

            /// Call CPUID and get this structure.
            pub fn get() -> $x {
                Info::get(InfoType::$x).into()
            }
        }
    );
}

derive_info!(VendorString);
derive_info!(Features);
derive_info!(Tlb);
derive_info!(Serial);
derive_info!(IntelExtended);
derive_info!(IntelFeatures);
derive_info!(IntelBrandString);
derive_info!(IntelBrandStringMore);
derive_info!(IntelBrandStringEnd);

derive_conversions!(Xsave0);
derive_conversions!(Xsave1);

impl Xsave0 {

    /// Call CPUID and get this structure.
    pub fn get() -> Self {
        Info::get_xsave(XsaveInfoType::Subf0).into()
    }

    /// Size of XSAVE region when all supported state components are
    /// stored.
    pub fn size_of_all(&self) -> u32 {
        self.info.ecx
    }

    /// Size of XSAVE region when all currently enabled state components are
    /// stored.
    pub fn size_of_current(&self) -> u32 {
        self.info.ebx
    }
}

impl Xsave1 {

    const XSAVEOPT      : u32 = (1 << 0);
    const COMPACT_FORM  : u32 = (1 << 1);
    const XGETBV        : u32 = (1 << 2);
    const XSAVES        : u32 = (1 << 3);

    /// Call CPUID and get this structure.
    pub fn get() -> Self {
        Info::get_xsave(XsaveInfoType::Subf1).into()
    }

    /// Whether XSAVEOPT instruction is supported.
    pub fn xsaveopt_supported(&self) -> bool {
        self.info.eax & Self::XSAVEOPT != 0
    }

    /// Whether compact form is supported. If so, XSAVEC and compact form of
    /// is supported XRSTOR.
    pub fn compact_form_supported(&self) -> bool {
        self.info.eax & Self::COMPACT_FORM != 0
    }

    /// Whether XGETBV instruction is supported.
    pub fn xgetbv_supported(&self) -> bool {
        self.info.eax & Self::XGETBV != 0
    }

    /// Whether XSAVES, XRSTORS, IA32_XSS MSR is supported.
    pub fn xsaves_supported(&self) -> bool {
        self.info.eax & Self::XSAVES != 0
    }

    /// Size of XSAVE area containing all the state components corresponding
    /// to bits currently set in XCR0 | IA32_XSS.
    pub fn xsaves_size_of_current(&self) -> u32 {
        self.info.ebx
    }
}

impl VendorString {

    /// Write vendor string (null-terminated) into the given array.
    pub fn vendor(&self, s: &mut [char; 13]) {
        s[12] = '\0'; // Null-terminate the string.

        s[0x00] = ((self.info.ebx & 0x000000FF) >> 0x00) as u8 as char;
        s[0x01] = ((self.info.ebx & 0x0000FF00) >> 0x08) as u8 as char;
        s[0x02] = ((self.info.ebx & 0x00FF0000) >> 0x10) as u8 as char;
        s[0x03] = ((self.info.ebx & 0xFF000000) >> 0x18) as u8 as char;
        s[0x04] = ((self.info.edx & 0x000000FF) >> 0x00) as u8 as char;
        s[0x05] = ((self.info.edx & 0x0000FF00) >> 0x08) as u8 as char;
        s[0x06] = ((self.info.edx & 0x00FF0000) >> 0x10) as u8 as char;
        s[0x07] = ((self.info.edx & 0xFF000000) >> 0x18) as u8 as char;
        s[0x08] = ((self.info.ecx & 0x000000FF) >> 0x00) as u8 as char;
        s[0x09] = ((self.info.ecx & 0x0000FF00) >> 0x08) as u8 as char;
        s[0x0A] = ((self.info.ecx & 0x00FF0000) >> 0x10) as u8 as char;
        s[0x0B] = ((self.info.ecx & 0xFF000000) >> 0x18) as u8 as char;
    }

    /// Maximal input value for basic CPUID information.
    pub fn max_value(&self) -> u32 {
        self.info.eax
    }
}

impl Features {

    /// Get brand index.
    pub fn brand_index(&self) -> u8 {
        (self.info.ebx & 0xFF) as u8
    }

    /// CLFLUSH line size. Value * 8 = cache line size in bytes.
    /// Used also by CLFLUSHOPT.
    pub fn clflush_line_size(&self) -> u8 {
        (self.info.ebx & 0xFF00 >> 8) as u8
    }

    /// Maximum number of addressable IDs for logical processors in this
    /// physical package. The nearest power-of-2 integer that is not smaller
    /// than EBX[23:16] is the number of unique initial APIC IDs reserved
    /// for addressing different logical processors in a physical package.
    /// This field is only valid if EDX.HTT (bit 28) is set.
    pub fn max_addressable_ids(&self) -> u8 {
        (self.info.ebx & 0xFF0000 >> 16) as u8
    }

    /// Get initial APIC ID.
    pub fn initial_apic_id(&self) -> u8 {
        (self.info.ebx & 0xFF000000 >> 24) as u8
    }

    pub fn extended_family_id(&self) -> u8 {
        (self.info.eax & 0b0000_1111_1111_0000_0000_0000_0000_0000 >> 20) as u8
    }

    pub fn extended_model_id(&self) -> u8 {
        (self.info.eax & 0b0000_0000_0000_1111_0000_0000_0000_0000 >> 16) as u8
    }

    pub fn processor_type(&self) -> u8 {
        (self.info.eax & 0b0000_0000_0000_0000_0011_0000_0000_0000 >> 12) as u8
    }

    pub fn family_id(&self) -> u8 {
        (self.info.eax & 0b0000_0000_0000_0000_0000_1111_0000_0000 >> 8) as u8
    }

    pub fn model(&self) -> u8 {
        (self.info.eax & 0b0000_0000_0000_0000_0000_0000_1111_0000 >> 4) as u8
    }

    pub fn stepping_id(&self) -> u8 {
        (self.info.eax & 0b0000_0000_0000_0000_0000_0000_0000_1111) as u8
    }

    /// Check if Local APIC is present.
    pub fn local_apic_is_present(&self) -> bool {
        self.info.edx & 0b0000_0000_0000_0000_0000_0001_0000_0000 != 0
    }

    /// Whether APIC supports one-shot operation using TSC deadline value.
    pub fn tsc_deadline_supported(&self) -> bool {
        self.info.ecx & (1 << 24) != 0
    }

    /// Whether XSAVE instruction family is supported.
    pub fn xsave_supported(&self) -> bool {
        self.info.ecx & (1 << 26) != 0
    }
}
