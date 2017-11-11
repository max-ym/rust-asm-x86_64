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

    Mode0                       = 0b000,
    Mode1                       = 0b001,
    Mode2                       = 0b010,
    Mode3                       = 0b011,
    Mode4                       = 0b100,
    Mode5                       = 0b101,
    Mode2_                      = 0b110,
    Mode3_                      = 0b111,
}

/// PIT access modes.
#[repr(u8)]
pub enum AccessMode {
    LatchCountValue = 0b00,
    LoByteOnly      = 0b01,
    HiByteOnly      = 0b10,
    LoHiByte        = 0b11,
}
