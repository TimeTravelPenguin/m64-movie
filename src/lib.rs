use bilge::{
    Bitsized,
    prelude::{DebugBits, DefaultBits, FromBits, Number, bitsize, u7},
};
use binrw::{BinRead, BinWrite, NullString, helpers::until_eof};

/// Validate a non-zero value that is only considered valid if
/// the extended version is a specific value. The result is true if the
/// value is all zeros or if the extended version matches one of the valid versions.
fn valid_only_if_ext_version_eq(extended_version: u8, valid_versions: &[u8], value: &[u8]) -> bool {
    valid_versions.contains(&extended_version) || value.iter().all(&|&b| b == 0)
}

#[derive(Debug, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[brw(little, magic = b"M64\x1A")]
#[brw(
    assert(self.version == 3, "Only version 3 is supported. Got: {}.", version),
    assert(
        valid_only_if_ext_version_eq(
            self.extended_version,
            &[1],
            &[self.extended_flags.wiivc_emulation_mode() as u8]
        ),
        "WiiVC emulation mode is only valid if extended version is 1."
    ),
    assert(
        valid_only_if_ext_version_eq(
            self.extended_version,
            &[1],
            &self.extended_data.authorship_info.to_le_bytes()
        ),
        "Authorship info is only valid if extended version is 1."
    ),
    assert(
        valid_only_if_ext_version_eq(
            self.extended_version,
            &[1],
            &self.extended_data.bruteforce_data.to_le_bytes()
        ),
        "Bruteforce data is only valid if extended version is 1."
    ),
    assert(
        valid_only_if_ext_version_eq(
            self.extended_version,
            &[1],
            &self.extended_data.rerecord_count_high.to_le_bytes()
        ),
        "Rerecord count high is only valid if extended version is 1."
    ),
)]
pub struct Movie {
    pub version: u32,                  // 0x004
    pub uid: u32,                      // 0x008
    pub vertical_interrupts: u32,      // 0x00C
    pub rerecord_count: u32,           // 0x010
    pub vis_per_second: u8,            // 0x014
    pub controller_count: u8,          // 0x015
    pub extended_version: u8,          // 0x016
    pub extended_flags: ExtendedFlags, // 0x017
    pub input_samples: u32,            // 0x018
    pub start_type: MovieStartType,    // 0x01C
    pub reserved01: Reserved<2>,       // 0x01E
    // TODO: Create ControllerFlags struct
    pub controller_flags: u32,       // 0x020
    pub extended_data: ExtendedData, // 0x024
    pub reserved02: Reserved<128>,   // 0x044

    #[brw(
        pad_size_to = 32,
        assert(rom_name.is_ascii(), "ROM name must be ASCII")
    )]
    pub rom_name: NullString, // 0x0C4
    pub rom_crc: u32,             // 0x0E4
    pub rom_country: u16,         // 0x0E8
    pub reserved03: Reserved<56>, // 0x0EA

    #[brw(
        pad_size_to = 64,
        assert(rom_name.is_ascii(), "Video plugin name must be ASCII")
    )]
    pub video_plugin: NullString, // 0x122

    #[brw(
        pad_size_to = 64,
        assert(rom_name.is_ascii(), "Sound plugin name must be ASCII")
    )]
    pub sound_plugin: NullString, // 0x162

    #[brw(
        pad_size_to = 64,
        assert(rom_name.is_ascii(), "Input plugin name must be ASCII")
    )]
    pub input_plugin: NullString, // 0x1A2

    #[brw(
        pad_size_to = 64,
        assert(rom_name.is_ascii(), "RSP plugin name must be ASCII")
    )]
    pub rsp_plugin: NullString, // 0x1E2

    #[brw(pad_size_to = 222)]
    pub author_name: NullString, // 0x222

    #[brw(pad_size_to = 256)]
    pub description: NullString, // 0x300

    #[brw(align_before = 0x400)]
    #[br(parse_with = until_eof)]
    pub inputs: Vec<ButtonState>, // 0x400
}

impl Movie {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, binrw::Error> {
        let mut cursor = std::io::Cursor::new(bytes);
        let raw_movie: Movie = Movie::read(&mut cursor)?;
        Ok(raw_movie)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, binrw::Error> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        self.write(&mut cursor)?;
        Ok(cursor.into_inner())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
pub struct Reserved<const T: usize> {
    pub reserved: [u8; T],
}

#[bitsize(8)]
#[derive(FromBits, DefaultBits, DebugBits, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[br(little, map = |raw: u8| Self::from(raw))]
#[bw(little, map = |s: &Self| s.value)]
pub struct ExtendedFlags {
    pub wiivc_emulation_mode: bool,
    pub reserved: u7,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct ExtendedData {
    pub authorship_info: u32,
    pub bruteforce_data: u32,
    pub rerecord_count_high: u32,
    pub reserved: Reserved<20>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub enum MovieStartType {
    #[brw(magic = 1u16)]
    Snapshot,
    #[brw(magic = 2u16)]
    PowerOn,
    #[brw(magic = 4u16)]
    EEPROM,
}

#[bitsize(32)]
#[derive(FromBits, DefaultBits, DebugBits, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[br(little, map = |raw: u32| Self::from(raw))]
#[bw(little, map = |s: &Self| s.value)]
pub struct ButtonState {
    pub dpad_right: bool,
    pub dpad_left: bool,
    pub dpad_down: bool,
    pub dpad_up: bool,
    pub start_btn: bool,
    pub z_btn: bool,
    pub b_btn: bool,
    pub a_btn: bool,
    pub c_right: bool,
    pub c_left: bool,
    pub c_down: bool,
    pub c_up: bool,
    pub trigger_right: bool,
    pub trigger_left: bool,
    pub reserved01: bool,
    pub reserved02: bool,
    _x_axis: u8,
    _y_axis: u8,
}

impl ButtonState {
    pub fn x_axis(&self) -> i8 {
        self._x_axis() as i8
    }

    pub fn set_x_axis(&mut self, value: i8) {
        self.set__x_axis(value as u8);
    }

    pub fn y_axis(&self) -> i8 {
        self._y_axis() as i8
    }

    pub fn set_y_axis(&mut self, value: i8) {
        self.set__y_axis(value as u8);
    }
}
