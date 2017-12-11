#[inline(always)]
pub fn bsf_u64(i: u64) -> Option<u64> {
    let zf  : u64;
    let r   : u64;

    unsafe { asm! (
        "
        bsf    $0, $0   \n
        setz   $1
        "
        : "=r"(r), "=r"(zf)
        : "r"(i)
        :: "intel"
    ); }

    if zf == 0 {
        Some(r)
    } else {
        None
    }
}

#[inline(always)]
pub fn bsf_u32(i: u32) -> Option<u32> {
    let zf  : u32;
    let r   : u32;

    unsafe { asm! (
        "
        bsf    $0, $0   \n
        setz   $1
        "
        : "=r"(r), "=r"(zf)
        : "r"(i)
        :: "intel"
    ); }

    if zf == 0 {
        Some(r)
    } else {
        None
    }
}

#[inline(always)]
pub fn bsf_u16(i: u16) -> Option<u16> {
    let zf  : u16;
    let r   : u16;

    unsafe { asm! (
        "
        bsf    $0, $0   \n
        setz   $1
        "
        : "=r"(r), "=r"(zf)
        : "r"(i)
        :: "intel"
    ); }

    if zf == 0 {
        Some(r)
    } else {
        None
    }
}

#[inline(always)]
pub fn bsr_u64(i: u64) -> Option<u64> {
    let zf  : u64;
    let r   : u64;

    unsafe { asm! (
        "
        bsr    $0, $0   \n
        setz   $1
        "
        : "=r"(r), "=r"(zf)
        : "r"(i)
        :: "intel"
    ); }

    if zf == 0 {
        Some(r)
    } else {
        None
    }
}

#[inline(always)]
pub fn bsr_u32(i: u32) -> Option<u32> {
    let zf  : u32;
    let r   : u32;

    unsafe { asm! (
        "
        bsr    $0, $0   \n
        setz   $1
        "
        : "=r"(r), "=r"(zf)
        : "r"(i)
        :: "intel"
    ); }

    if zf == 0 {
        Some(r)
    } else {
        None
    }
}

#[inline(always)]
pub fn bsr_u16(i: u16) -> Option<u16> {
    let zf  : u16;
    let r   : u16;

    unsafe { asm! (
        "
        bsr    $0, $0   \n
        setz   $1
        "
        : "=r"(r), "=r"(zf)
        : "r"(i)
        :: "intel"
    ); }

    if zf == 0 {
        Some(r)
    } else {
        None
    }
}

#[inline(always)]
pub fn bsf_i64(i: i64) -> Option<i64> {
    match bsf_u64(i as u64) {
        Some(v) => Some(v as i64),
        None    => None
    }
}

#[inline(always)]
pub fn bsf_i32(i: i32) -> Option<i32> {
    match bsf_u32(i as u32) {
        Some(v) => Some(v as i32),
        None    => None
    }
}

#[inline(always)]
pub fn bsf_i16(i: i16) -> Option<i16> {
    match bsf_u16(i as u16) {
        Some(v) => Some(v as i16),
        None    => None
    }
}

#[inline(always)]
pub fn bsr_i64(i: i64) -> Option<i64> {
    match bsr_u64(i as u64) {
        Some(v) => Some(v as i64),
        None    => None
    }
}

#[inline(always)]
pub fn bsr_i32(i: i32) -> Option<i32> {
    match bsr_u32(i as u32) {
        Some(v) => Some(v as i32),
        None    => None
    }
}

#[inline(always)]
pub fn bsr_i16(i: i16) -> Option<i16> {
    match bsr_u16(i as u16) {
        Some(v) => Some(v as i16),
        None    => None
    }
}
