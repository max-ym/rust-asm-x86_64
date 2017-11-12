/// Command register of PIT.
const CMD_REG: u16 = 0x43;

/// Base port number for channels. Chan0 port is 0x40, Chan1 port is 0x41 etc.
const CH_BASE: u16 = 0x40;

/// Port that has bit #0 that controls channel 2 gate.
const CH2_GATE_PORT: u16 = 0x61;

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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub enum AccessMode {
    LoByteOnly      = 0b01,
    HiByteOnly      = 0b10,
    LoHiByte        = 0b11,
}

/// Information about channel settings.
#[derive(Clone, Copy)]
struct ChannelInfo {
    pub access      : AccessMode,
    pub operating   : OperatingMode,
    pub reload      : u16,
}

/// PIT interface.
pub struct Pit {
    ch0     : ChannelInfo,
    //ch1     : ChannelInfo, // unimplemented on modern PCs.
    ch2     : ChannelInfo,
    ch2gate : bool,

    // Settings to apply on next write to command register for individual
    // channels.
    ch0_pending : ChannelInfo,
    ch2_pending : ChannelInfo,
}

/// Status byte read from corresponding channel port.
#[repr(packed)]
pub struct StatusByte {
    val     : u8,
}

/// Port for PIT command register.
fn cmd_port() -> ::port::Port {
    ::port::Port::number(CMD_REG)
}

impl Channel {

    /// Port for this channel.
    pub fn port(&self) -> ::port::Port {
        ::port::Port::number(CH_BASE + *self as u8 as u16)
    }

    /// Current count value in lo/hi access mode.
    ///
    /// # Safety
    /// Use only for lo/hi access mode, otherwise bad data will be read.
    /// No other commands to PIT are allowed while running this function.
    /// Caller must ensure that other CPUs don't access PIT in the meantime
    /// and that no interrupt occurs that can try to access PIT.
    pub unsafe fn current_count(&self) -> u16 {
        let chan_port = self.port();

        // Send latch command for this channel.
        cmd_port().out_u8((*self as u8) << 6);

        // Read lo and hi bytes from port.
        let lo = chan_port.in_u8() as u16;
        let hi = chan_port.in_u8() as u16;
        lo | (hi << 8)
    }

    /// Current count value in lo OR hi access mode. Function
    /// returns appropriate byte which is either lo or hi according to
    /// current mode.
    ///
    /// # Safety
    /// Use only for lo OR hi access mode, otherwise bad data will be read
    /// and next reads will get corrupted.
    /// No other commands to PIT are allowed while running this function.
    /// Caller must ensure that other CPUs don't access PIT in the meantime
    /// and that no interrupt occurs that can try to access PIT.
    pub unsafe fn current_count_byte(&self) -> u8 {
        self.port().in_u8()
    }

    /// Set new reload value in lo_hi access mode.
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

    /// Set new reload value in lo or hi access mode.
    ///
    /// # Safety
    /// Use only for lo OR hi access mode, otherwise bad data will be set
    /// and next read/write operations will get corrupted.
    /// No other commands to PIT are allowed while running this function.
    /// Caller must ensure that other CPUs don't access PIT in the meantime
    /// and that no interrupt occurs that can try to access PIT.
    pub unsafe fn set_reload_byte(&mut self, c: u8) {
        self.port().out_u8(c)
    }
}

macro_rules! pit_ch_impl {
    ($channel:ident, $ch:ident,
            $pending:ident, $set_access:ident, $set_operating:ident,
            $commit_settings:ident, $commit_count:ident,
            $commit_all:ident) => (

    /// Change access mode for channel 0.
    pub fn $set_access(&mut self, mode: AccessMode) {
        self.$pending.access = mode;
    }

    /// Change operating mode for channel 0.
    pub fn $set_operating(&mut self, mode: OperatingMode) {
        self.$pending.operating = mode;
    }

    /// Commit pending settings to the channel 0.
    pub fn $commit_settings(&mut self) {
        use self::Channel::$channel;

        let status = StatusByte::new(Some($channel),
                Some(self.$pending.access), self.$pending.operating);

        cmd_port().out_u8(status.into());

        self.$ch = self.$pending;
    }

    /// Commit pending initial count value to channel 0.
    ///
    /// Value is sent according to current access mode. For example,
    /// if value contains set bits in hi and lo bytes, but access mode
    /// allows storing only hi byte, the value of lo byte will not be
    /// updated. However, current value of this Pit interface is updated
    /// to the same value as the PIT taking into account current access mode.
    /// Value of pending count of the interface is not changed and lo/hi parts
    /// are not discarded even when they aren't updated in PIT due to access
    /// mode.
    pub fn $commit_count(&mut self) {
        use self::AccessMode::*;
        use self::Channel::$channel;

        unsafe { match self.$ch.access {

            LoByteOnly => {
                let reload = self.$pending.reload & 0x00FF;
                $channel.set_reload_byte(reload as u8);

                self.$ch.reload = reload;
            },

            HiByteOnly => {
                let reload = self.$pending.reload & 0xFF00;
                $channel.set_reload_byte((reload >> 8) as _);

                self.$ch.reload = reload;
            },

            LoHiByte => {
                $channel.set_reload(self.$pending.reload);
                self.$ch.reload = self.$pending.reload;
            }
        }}
    }

    /// Commit all settings and reset initial counter.
    pub fn $commit_all(&mut self) {
        self.$commit_settings();
        self.$commit_count();
    }
    );
}

impl Pit {

    /// Create new PIT interface. Default data may not be same as current PIT
    /// settings.
    ///
    /// # Safety
    /// Caller must understand that this interface will hold data that may
    /// not correspond to current PIT settings. This may lead to misbehaviour
    /// and caller can update PIT settings if needed.
    pub unsafe fn new_no_sync() -> Self {
        use self::AccessMode::LoHiByte;
        use self::OperatingMode::RateGenerator;
        use self::OperatingMode::SoftwareTriggeredStrobe;

        let ch0 = ChannelInfo {
            access      : LoHiByte,
            operating   : RateGenerator,
            reload      : 0, // IRQ0 with 18.2065 Hz.
        };

        let ch2 = ChannelInfo {
            access      : LoHiByte,
            operating   : SoftwareTriggeredStrobe,
            reload      : 1,
        };

        Pit {
            ch0 : ch0,
            ch2 : ch2,

            ch2gate : false,

            ch0_pending : ch0,
            ch2_pending : ch2,
        }
    }

    pit_ch_impl!(Channel0, ch0, ch0_pending, ch0_set_access,
            ch0_set_operating, ch0_commit_settings, ch0_commit_count,
            ch0_commit_all);
}

impl StatusByte {

    /// Create new status byte with given settings.
    ///
    /// Provided channel value is optional. If no channel is given,
    /// this status byte will select a read-back command.
    ///
    /// Access is optional too. If no access given then latch command
    /// will be seleted.
    pub fn new_with_bcd(ch: Option<Channel>, access: Option<AccessMode>,
            op: OperatingMode, bcd: bool) -> Self {
        StatusByte {
            val : {
                (match ch {
                    Some(t) => t as u8,
                    None    => 0b11 // Read-back command.
                }                           << 6)
                | (match access {
                    Some(t) => t as u8,
                    None    => 0b00 // Latch command.
                }                           << 4)
                | ((op as u8)               << 1)
                | (if bcd { 1 } else { 0 }  << 0)
            }
        }
    }

    /// Create new status byte with given settings.
    ///
    /// Provided channel value is optional. If no channel is given,
    /// this status byte will select a read-back command.
    ///
    /// Access is optional too. If no access given then latch command
    /// will be seleted.
    pub fn new(ch: Option<Channel>, access: Option<AccessMode>,
            op: OperatingMode) -> Self {
        Self::new_with_bcd(ch, access, op, false)
    }

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

impl Into<u8> for StatusByte {

    fn into(self) -> u8 {
        self.val
    }
}
