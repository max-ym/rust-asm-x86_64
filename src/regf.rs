/// General purpose registers file.
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct GeneralPurpose {
    pub rax     : u64,
    pub rbx     : u64,
    pub rcx     : u64,
    pub rdx     : u64,
    pub rsi     : u64,
    pub rdi     : u64,

    pub rbp     : u64,
    pub rsp     : u64,

    pub r8      : u64,
    pub r9      : u64,
    pub r10     : u64,
    pub r11     : u64,
    pub r12     : u64,
    pub r13     : u64,
    pub r14     : u64,
    pub r15     : u64,
}

/// Floating point MMX stack registers.
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Mmx {
    pub mmx0    : u64,
    pub mmx1    : u64,
    pub mmx2    : u64,
    pub mmx3    : u64,
    pub mmx4    : u64,
    pub mmx5    : u64,
    pub mmx6    : u64,
    pub mmx7    : u64,
}

/// Floating point 80-bit registers.
///
/// 64-bit parts and 16-bit parts of the registers are stored seperately in
/// this struct to optimize access.
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Floating {
    pub st0     : u64,
    pub st1     : u64,
    pub st2     : u64,
    pub st3     : u64,
    pub st4     : u64,
    pub st5     : u64,
    pub st6     : u64,
    pub st7     : u64,

    pub st0u16  : u16,
    pub st1u16  : u16,
    pub st2u16  : u16,
    pub st3u16  : u16,
    pub st4u16  : u16,
    pub st5u16  : u16,
    pub st6u16  : u16,
    pub st7u16  : u16,
}

/// FLAGS register and instruction pointer.
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct State {
    pub rip     : u64,
    pub flags   : u32,
}

/// Segment registers.
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Segments {
    pub cs      : u16,
    pub ds      : u16,
    pub es      : u16,
    pub ss      : u16,
    pub fs      : u16,
    pub gs      : u16,
}

/// SSE registers.
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Sse {
    pub xmm0l   : u64,
    pub xmm0h   : u64,
    pub xmm1l   : u64,
    pub xmm1h   : u64,
    pub xmm2l   : u64,
    pub xmm2h   : u64,
    pub xmm3l   : u64,
    pub xmm3h   : u64,
    pub xmm4l   : u64,
    pub xmm4h   : u64,
    pub xmm5l   : u64,
    pub xmm5h   : u64,
    pub xmm6l   : u64,
    pub xmm6h   : u64,
    pub xmm7l   : u64,
    pub xmm7h   : u64,
    pub xmm8l   : u64,
    pub xmm8h   : u64,
    pub xmm9l   : u64,
    pub xmm9h   : u64,
    pub xmm10l  : u64,
    pub xmm10h  : u64,
    pub xmm11l  : u64,
    pub xmm11h  : u64,
    pub xmm12l  : u64,
    pub xmm12h  : u64,
    pub xmm13l  : u64,
    pub xmm13h  : u64,
    pub xmm14l  : u64,
    pub xmm14h  : u64,
    pub xmm15l  : u64,
    pub xmm15h  : u64,
}
