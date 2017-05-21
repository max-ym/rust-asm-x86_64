#![allow(dead_code)]

/// Port to receive or send data through.
pub struct Port {
    p   : u16
}

impl From<u16> for Port {

    fn from(p: u16) -> Port { Port { p:p } }
}

impl From<i16> for Port {

    fn from(p: i16) -> Port { Port { p:p as u16 } }
}

impl Port {

    /// Create port with given ID.
    pub fn number(p: u16) -> Self { Port { p:p } }

    #[inline(always)]
    pub fn out_u8(&self, data: u8) {
        unsafe { asm!(
            "out    dx, al"
            : // No output
            : "{al}"(data), "{dx}"(self.p)
            :: "intel"
        ); }
    }

    #[inline(always)]
    pub fn out_i8(&self, data: i8) {
        self.out_u8(data as u8)
    }

    #[inline(always)]
    pub fn out_u16(&self, data: u16) {
        unsafe { asm!(
            "out    dx, ax"
            : // No output
            : "{ax}"(data), "{dx}"(self.p)
            :: "intel"
        ); }
    }

    #[inline(always)]
    pub fn out_i16(&self, data: i16) {
        self.out_u16(data as u16)
    }

    #[inline(always)]
    pub fn out_u32(&self, data: u32) {
        unsafe { asm!(
            "out    dx, eax"
            : // No output
            : "{eax}"(data), "{dx}"(self.p)
            :: "intel"
        ); }
    }

    #[inline(always)]
    pub fn out_i32(&self, data: i32) {
        self.out_u32(data as u32)
    }

    #[inline(always)]
    pub fn in_u8(&self) -> u8 {
        let result;
        unsafe { asm!(
            "in     al, dx"
            : "={al}"(result)
            : "{dx}"(self.p)
            :: "intel"
        ); }
        result
    }

    #[inline(always)]
    pub fn in_i8(&self) -> i8 {
        self.in_u8() as i8
    }

    #[inline(always)]
    pub fn in_u16(&self) -> u16 {
        let result;
        unsafe { asm!(
            "in     ax, dx"
            : "={ax}"(result)
            : "{dx}"(self.p)
            :: "intel"
        ); }
        result
    }

    #[inline(always)]
    pub fn in_i16(&self) -> i16 {
        self.in_u16() as i16
    }

    #[inline(always)]
    pub fn in_u32(&self) -> u32 {
        let result;
        unsafe { asm!(
            "in     eax, dx"
            : "={eax}"(result)
            : "{dx}"(self.p)
            :: "intel"
        ); }
        result
    }

    #[inline(always)]
    pub fn in_i32(&self) -> i32 {
        self.in_u32() as i32
    }
}
