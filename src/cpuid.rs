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

    IntelExtended       = 0x8000_0000,
    IntelFeatures       = 0x8000_0001,
    IntelBrandString    = 0x8000_0002,
    IntelBrandStringMore= 0x8000_0003,
    IntelBrandStringEnd = 0x8000_0004,
}

impl Info {

    /// Run CPUID instruction to query information.
    #[inline(always)]
    pub fn get(info: InfoType) -> Self {
        Self::get_by_code(info as u32)
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
}

macro_rules! derive_info {
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
}
