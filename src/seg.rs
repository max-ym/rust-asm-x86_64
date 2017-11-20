#![allow(dead_code)]

/// Macro to create getter/setter functions for segment registers.
/// First argument always must be a name of a register. Also this
/// will be the name of getter. The other argument is a name of a setter.
macro_rules! new {
    ($x:ident, $y:ident) => {
        #[inline(always)]
        pub fn $x() -> u16 {
            let val;
            unsafe {
                asm!(
                    "mov $0, $x"
                    : "=r"(val)
                    ::: "intel"
                );
            }
            val
        }

        #[inline(always)]
        pub fn $y(val: u16) {
            unsafe {
                asm!(
                    "mov $x, $0"
                    :: "r"(val)
                    :: "intel"
                );
            }
        }
    };
}

/// Stack segment register.
new!(ss, set_ss);

/// Data segment register.
new!(ds, set_ds);

/// Extra segment register.
new!(es, set_es);

/// Extra "F" segment register.
new!(fs, set_fs);

/// Extra "G" segment register.
new!(gs, set_gs);

/// Task State Segment.
#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct Tss {
    pub _resv0      : u32,

    pub rsp0        : u64,
    pub rsp1        : u64,
    pub rsp2        : u64,

    pub _resv1      : u64,

    pub ist1        : u64,
    pub ist2        : u64,
    pub ist3        : u64,
    pub ist4        : u64,
    pub ist5        : u64,
    pub ist6        : u64,
    pub ist7        : u64,

    pub _resv2      : u64,
    pub _resv3      : u16,

    pub iomap_offset: u16,
}
