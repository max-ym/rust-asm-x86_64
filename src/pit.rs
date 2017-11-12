/// Command register of PIT.
const CMD_REG: u16 = 0x43;

/// Base port number for channels. Chan0 port is 0x40, Chan1 port is 0x41 etc.
const CH_BASE: u16 = 0x40;

/// The channel of PIT.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Channel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
}

/// PIT operating modes.
#[repr(u8)]
pub enum OperatingMode {
    InterruptOnTerminalCount    = 0b000,
    HwRetriggerableOneShot      = 0b001,
    RateGenerator               = 0b010,
    SquareWaveGenerator         = 0b011,
    SoftwareTriggeredStrobe     = 0b100,
    HardwareTriggeredStrobe     = 0b101,
    RateGenerator2              = 0b110,
    SquareWaveGenerator2        = 0b111,
}

/// PIT access modes.
#[repr(u8)]
pub enum AccessMode {
    LatchCountValue = 0b00,
    LoByteOnly      = 0b01,
    HiByteOnly      = 0b10,
    LoHiByte        = 0b11,
}

/// Status byte read from corresponding channel port.
#[repr(packed)]
pub struct StatusByte {
    val     : u8,
}

impl Channel {

    /// Port for this channel.
    pub fn port(&self) -> ::port::Port {
        ::port::Port::number(CH_BASE + *self as u8 as u16)
    }

    /// Port for PIT command register.
    fn cmd_port() -> ::port::Port {
        ::port::Port::number(CMD_REG)
    }

    /// Current count value.
    ///
    /// # Safety
    /// Use only for lo/hi access mode, otherwise bad data will be read.
    /// No other commands to PIT are allowed while running this function.
    /// Caller must ensure that other CPUs don't access PIT in the meantime
    /// and that no interrupt occurs that can try to access PIT.
    pub unsafe fn current_count(&self) -> u16 {
        let chan_port = self.port();

        // Send latch command for this channel.
        Self::cmd_port().out_u8((*self as u8) << 6);

        // Read lo and hi bytes from port.
        let lo = chan_port.in_u8() as u16;
        let hi = chan_port.in_u8() as u16;
        lo | (hi << 8)
    }

    /// Set new reload value.
    ///
    /// # Safety
    /// Use only for lo/hi access mode, otherwise bad data will be wrote.
    /// No other commands to PIT are allowed while running this function.
    /// Caller must ensure that other CPUs don't access PIT in the meantime
    /// and that no interrupt occurs that can try to access PIT.
    pub unsafe fn set_reload(&mut self, c: u16) {
        let port = self.port();

        // Send lo and hi bytes.
        port.out_u8((c >> 0) as _);
        port.out_u8((c >> 8) as _);
    }
}

impl StatusByte {

    /// Read status byte from given channel.
    pub fn read_from(chan: Channel) -> Self {
        unimplemented!()
    }

    pub fn output_pin_state(&self) -> bool {
        self.val & 0b1000_0000 != 0
    }

    pub fn null_count_flags(&self) -> bool {
        self.val & 0b0100_0000 != 0
    }

    pub fn access_mode(&self) -> AccessMode {
        let val = self.val & 0b0011_0000;
        let val = val >> 4;
        unsafe { ::core::mem::transmute(val) }
    }

    pub fn operating_mode(&self) -> OperatingMode {
        let val = self.val & 0b0000_1110;
        let val = val >> 1;
        unsafe { ::core::mem::transmute(val) }
    }

    pub fn is_bcd_mode(&self) -> bool {
        self.val & 0b0000_0001 == 1
    }

    pub fn is_binary_mode(&self) -> bool {
        self.val & 0b0000_0001 == 0
    }
}
