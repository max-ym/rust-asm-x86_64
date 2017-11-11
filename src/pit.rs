/// The channel of PIT.
pub enum Channel {
    Channel0,
    Channel1,
    Channel2,
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

impl StatusByte {

    /// Read status byte from given channel.
    pub fn read_from(chan: Channel) -> Self {
        unimplemented!()
    }
}
