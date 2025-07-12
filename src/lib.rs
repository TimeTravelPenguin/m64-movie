#![doc = include_str!("../README.md")]

use std::fmt::{self, Debug};

use bilge::{
    Bitsized,
    prelude::{DebugBits, DefaultBits, FromBits, Number, bitsize, u7, u20},
};
use binrw::{BinRead, BinWrite, NullString, helpers::until_eof};

/// Validate a non-zero value that is only considered valid if
/// the extended version is a specific value. The result is true if the
/// value is all zeros or if the extended version matches one of the valid versions.
fn valid_only_if_ext_version_eq(extended_version: u8, valid_versions: &[u8], value: &[u8]) -> bool {
    valid_versions.contains(&extended_version) || value.iter().all(&|&b| b == 0)
}

/// A Mupen64 movie file.
///
/// Only version 3 is supported. Please refer to the
/// [file format documentation](https://tasvideos.org/EmulatorResources/Mupen/M64) for more details.
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
    /// The version of the Mupen64 movie format.
    pub version: u32, // 0x004

    /// The unique identifier for the movie.
    pub uid: u32, // 0x008

    /// The number of vertical interrupts in the movie.
    pub vertical_interrupts: u32, // 0x00C

    /// The number of rerecords in the movie.
    pub rerecord_count: u32, // 0x010

    /// The number of vertical interrupts per second.
    pub vis_per_second: u8, // 0x014

    /// The number of controllers used in the movie.
    pub controller_count: u8, // 0x015

    /// The extended version of the movie format. On versions of Mupen64 movies
    /// created with mupen <1.1.9, this value is always 0.
    pub extended_version: u8, // 0x016

    /// Extended flags for the movie. This is only valid if the extended version is 1.
    pub extended_flags: ExtendedFlags, // 0x017

    /// The number of input samples for any controller in the movie.
    pub controller_input_samples: u32, // 0x018

    /// The start type of the movie, indicating how the movie begins.
    pub start_type: MovieStartType, // 0x01C

    /// Reserved space.
    pub reserved01: Reserved<2>, // 0x01E

    /// Flags indicating the presence and capabilities of controllers.
    pub controller_flags: ControllerFlags, // 0x020

    /// Extended data for the movie, which is only valid if the extended version is non-zero.
    pub extended_data: ExtendedData, // 0x024

    /// Reserved space.
    pub reserved02: Reserved<128>, // 0x044

    /// The internal name of the ROM used in the movie. This value is taken
    /// directly from the ROM. Should be a 32-byte ASCII string.
    #[brw(
        pad_size_to = 32,
        assert(rom_name.is_ascii(), "ROM name must be ASCII")
    )]
    pub rom_name: NullString, // 0x0C4

    /// The CRC32 checksum of the ROM used in the movie. This value is taken
    /// directly from the ROM.
    pub rom_crc32: u32, // 0x0E4

    /// The country code of the ROM used in the movie. This value is taken
    /// directly from the ROM.
    pub rom_country: u16, // 0x0E8

    /// Reserved space.
    pub reserved03: Reserved<56>, // 0x0EA

    /// The name of the video plugin used in the movie. This value is
    /// taken directly from the plugin. Should be a 64-byte ASCII string.
    #[brw(
        pad_size_to = 64,
        assert(video_plugin.is_ascii(), "Video plugin name must be ASCII")
    )]
    pub video_plugin: NullString, // 0x122

    /// The name of the sound plugin used in the movie. This value is
    /// taken directly from the plugin. Should be 64-byte ASCII string.
    #[brw(
        pad_size_to = 64,
        assert(sound_plugin.is_ascii(), "Sound plugin name must be ASCII")
    )]
    pub sound_plugin: NullString, // 0x162

    /// The name of the input plugin used in the movie. This value is
    /// taken directly from the plugin. Should be 64-byte ASCII string.
    #[brw(
        pad_size_to = 64,
        assert(input_plugin.is_ascii(), "Input plugin name must be ASCII")
    )]
    pub input_plugin: NullString, // 0x1A2

    /// The name of the RSP plugin used in the movie. This value is
    /// taken directly from the plugin. Should be 64-byte ASCII string.
    #[brw(
        pad_size_to = 64,
        assert(rsp_plugin.is_ascii(), "RSP plugin name must be ASCII")
    )]
    pub rsp_plugin: NullString, // 0x1E2

    /// Author name info for the movie. Should be 222-byte UTF-8 string.
    #[brw(pad_size_to = 222)]
    pub author_name: NullString, // 0x222

    /// Author description info for the movie. Should be 256-byte UTF-8 string.
    #[brw(pad_size_to = 256)]
    pub description: NullString, // 0x300

    /// Controller inputs for the movie.
    #[brw(align_before = 0x400)]
    #[br(parse_with = until_eof)]
    pub inputs: Vec<ControllerState>, // 0x400
}

impl TryFrom<&[u8]> for Movie {
    type Error = binrw::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = std::io::Cursor::new(bytes);
        Movie::read(&mut cursor)
    }
}

impl Movie {
    /// Parses a Mupen64 movie from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> BinResult<Self> {
        Movie::try_from(bytes)
    }
    }

    /// Writes the Mupen64 movie to bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, binrw::Error> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        self.write(&mut cursor)?;
        Ok(cursor.into_inner())
    }

    /// Returns an iterator over the controller states. Each iteration yields an iterator
    /// containing the states of all controllers for that frame.
    ///
    /// Note that the index of each controller is determined by the game.
    /// So, the first controller in a frame may not be "Player 1" in the game.
    pub fn controller_inputs_stream(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = &ControllerState>> {
        self.inputs
            .chunks(self.controller_count as usize)
            .map(move |chunk| chunk.iter())
    }
}

/// A struct implementing `BinRead` and `BinWrite` for reserved space in bytes.
#[derive(Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
pub struct Reserved<const T: usize> {
    /// A span of `T` reserved bytes.
    pub reserved: [u8; T],
}

impl<const T: usize> Debug for Reserved<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reserved({} bytes)", T)
    }
}

/// A 1-byte structure for extended flags found at offset 0x017 in the Mupen64 movie header.
#[bitsize(8)]
#[derive(FromBits, DefaultBits, DebugBits, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[br(little, map = |raw: u8| Self::from(raw))]
#[bw(little, map = |s: &Self| s.value)]
pub struct ExtendedFlags {
    pub wiivc_emulation_mode: bool,
    pub reserved: u7,
}

/// A 4-byte structure for controller flags found at offset 0x020 in the Mupen64 movie header.
#[bitsize(32)]
#[derive(FromBits, DefaultBits, DebugBits, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[br(little, map = |raw: u32| Self::from(raw))]
#[bw(little, map = |s: &Self| s.value)]
pub struct ControllerFlags {
    /// Controller 01 present flag.
    pub controller_01_present: bool,
    /// Controller 02 present flag.
    pub controller_02_present: bool,
    /// Controller 03 present flag.
    pub controller_03_present: bool,
    /// Controller 04 present flag.
    pub controller_04_present: bool,
    /// Controller 01 memory pack flag.
    pub controller_01_has_mempak: bool,
    /// Controller 02 memory pack flag.
    pub controller_02_has_mempak: bool,
    /// Controller 03 memory pack flag.
    pub controller_03_has_mempak: bool,
    /// Controller 04 memory pack flag.
    pub controller_04_has_mempak: bool,
    /// Controller 01 rumble pack flag.
    pub controller_01_has_rumblepak: bool,
    /// Controller 02 rumble pack flag.
    pub controller_02_has_rumblepak: bool,
    /// Controller 03 rumble pack flag.
    pub controller_03_has_rumblepak: bool,
    /// Controller 04 rumble pack flag.
    pub controller_04_has_rumblepak: bool,
    /// Remaining reserved space.
    pub reserved: u20,
}

impl ControllerFlags {
    /// Returns the number of controllers present in the movie.
    pub fn num_controllers_present(&self) -> u8 {
        self.controller_01_present() as u8
            + self.controller_02_present() as u8
            + self.controller_03_present() as u8
            + self.controller_04_present() as u8
    }
}

/// A 32-byte structure for extended data found at offset 0x024 in the Mupen64 movie header.
#[derive(Debug, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct ExtendedData {
    /// Special authorship information.
    pub authorship_info: u32,
    /// Data regarding bruteforcing.
    pub bruteforce_data: u32,
    /// The high word of the rerecord count.
    pub rerecord_count_high: u32,
    /// The remaining reserved space.
    pub reserved: Reserved<20>,
}

/// An enum representing the start type of a Mupen64 movie.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub enum MovieStartType {
    /// The movie starts from a snapshot.
    #[brw(magic = 1u16)]
    Snapshot,
    /// The movie starts from a power-on state.
    #[brw(magic = 2u16)]
    PowerOn,
    /// The movie starts from EEPROM.
    #[brw(magic = 4u16)]
    EEPROM,
}

/// An enum representing the buttons on a Mupen64 controller.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ControllerButton {
    /// The right directional pad button.
    DPadRight,
    /// The left directional pad button.
    DPadLeft,
    /// The down directional pad button.
    DPadDown,
    /// The up directional pad button.
    DPadUp,
    /// The start button.
    Start,
    /// The Z button.
    Z,
    /// The B button.
    B,
    /// The A button.
    A,
    /// The C-right button.
    CRight,
    /// The C-left button.
    CLeft,
    /// The C-down button.
    CDown,
    /// The C-up button.
    CUp,
    /// The right trigger button.
    TriggerRight,
    /// The left trigger button.
    TriggerLeft,
    /// Reserved button 01.
    Reserved01,
    /// Reserved button 02.
    Reserved02,
}

/// A 4-byte structure representing controller input.
#[bitsize(32)]
#[derive(FromBits, DefaultBits, DebugBits, Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
#[br(little, map = |raw: u32| Self::from(raw))]
#[bw(little, map = |s: &Self| s.value)]
pub struct ControllerState {
    /// The right directional pad button.
    pub dpad_right: bool,
    /// The left directional pad button.
    pub dpad_left: bool,
    /// The down directional pad button.
    pub dpad_down: bool,
    /// The up directional pad button.
    pub dpad_up: bool,
    /// The start button.
    pub start_btn: bool,
    /// The Z button.
    pub z_btn: bool,
    /// The B button.
    pub b_btn: bool,
    /// The A button.
    pub a_btn: bool,
    /// The C-right button.
    pub c_right: bool,
    /// The C-left button.
    pub c_left: bool,
    /// The C-down button.
    pub c_down: bool,
    /// The C-up button.
    pub c_up: bool,
    /// The right trigger button.
    pub trigger_right: bool,
    /// The left trigger button.
    pub trigger_left: bool,
    /// Reserved button 01.
    pub reserved01: bool,
    /// Reserved button 02.
    pub reserved02: bool,
    /// The analog x-axis value, represented as an 8-bit unsigned integer.
    _x_axis: u8,
    /// The analog y-axis value, represented as an 8-bit unsigned integer.
    _y_axis: u8,
}

impl ControllerState {
    /// Get the analog x-axis value.
    pub fn x_axis(&self) -> i8 {
        self._x_axis() as i8
    }

    /// Set the analog x-axis value.
    pub fn set_x_axis(&mut self, value: i8) {
        self.set__x_axis(value as u8);
    }

    /// Get the analog y-axis value.
    pub fn y_axis(&self) -> i8 {
        self._y_axis() as i8
    }

    /// Set the analog y-axis value.
    pub fn set_y_axis(&mut self, value: i8) {
        self.set__y_axis(value as u8);
    }

    /// Returns a tuple of the x and y axis values.
    pub fn axis(&self) -> (i8, i8) {
        (self.x_axis(), self.y_axis())
    }

    /// Set the analog x and y axis values.
    pub fn set_axis(&mut self, x: i8, y: i8) {
        self.set_x_axis(x);
        self.set_y_axis(y);
    }

    /// Set a button as pressed.
    pub fn set(&mut self, button: ControllerButton) {
        match button {
            ControllerButton::DPadRight => self.set_dpad_right(true),
            ControllerButton::DPadLeft => self.set_dpad_left(true),
            ControllerButton::DPadDown => self.set_dpad_down(true),
            ControllerButton::DPadUp => self.set_dpad_up(true),
            ControllerButton::Start => self.set_start_btn(true),
            ControllerButton::Z => self.set_z_btn(true),
            ControllerButton::B => self.set_b_btn(true),
            ControllerButton::A => self.set_a_btn(true),
            ControllerButton::CRight => self.set_c_right(true),
            ControllerButton::CLeft => self.set_c_left(true),
            ControllerButton::CDown => self.set_c_down(true),
            ControllerButton::CUp => self.set_c_up(true),
            ControllerButton::TriggerRight => self.set_trigger_right(true),
            ControllerButton::TriggerLeft => self.set_trigger_left(true),
            ControllerButton::Reserved01 => self.set_reserved01(true),
            ControllerButton::Reserved02 => self.set_reserved02(true),
        }
    }

    /// Set a button as not pressed.
    pub fn unset(&mut self, button: ControllerButton) {
        match button {
            ControllerButton::DPadRight => self.set_dpad_right(false),
            ControllerButton::DPadLeft => self.set_dpad_left(false),
            ControllerButton::DPadDown => self.set_dpad_down(false),
            ControllerButton::DPadUp => self.set_dpad_up(false),
            ControllerButton::Start => self.set_start_btn(false),
            ControllerButton::Z => self.set_z_btn(false),
            ControllerButton::B => self.set_b_btn(false),
            ControllerButton::A => self.set_a_btn(false),
            ControllerButton::CRight => self.set_c_right(false),
            ControllerButton::CLeft => self.set_c_left(false),
            ControllerButton::CDown => self.set_c_down(false),
            ControllerButton::CUp => self.set_c_up(false),
            ControllerButton::TriggerRight => self.set_trigger_right(false),
            ControllerButton::TriggerLeft => self.set_trigger_left(false),
            ControllerButton::Reserved01 => self.set_reserved01(false),
            ControllerButton::Reserved02 => self.set_reserved02(false),
        }
    }

    /// Return whether a button is pressed.
    pub fn is_set(&self, button: ControllerButton) -> bool {
        match button {
            ControllerButton::DPadRight => self.dpad_right(),
            ControllerButton::DPadLeft => self.dpad_left(),
            ControllerButton::DPadDown => self.dpad_down(),
            ControllerButton::DPadUp => self.dpad_up(),
            ControllerButton::Start => self.start_btn(),
            ControllerButton::Z => self.z_btn(),
            ControllerButton::B => self.b_btn(),
            ControllerButton::A => self.a_btn(),
            ControllerButton::CRight => self.c_right(),
            ControllerButton::CLeft => self.c_left(),
            ControllerButton::CDown => self.c_down(),
            ControllerButton::CUp => self.c_up(),
            ControllerButton::TriggerRight => self.trigger_right(),
            ControllerButton::TriggerLeft => self.trigger_left(),
            ControllerButton::Reserved01 => self.reserved01(),
            ControllerButton::Reserved02 => self.reserved02(),
        }
    }

    /// Toggle whether a button is pressed.
    pub fn toggle(&mut self, button: ControllerButton) {
        if self.is_set(button) {
            self.unset(button);
        } else {
            self.set(button);
        }
    }

    /// Get a vector of all buttons that are currently pressed.
    pub fn get_pressed(&self) -> Vec<ControllerButton> {
        let buttons = [
            ControllerButton::DPadRight,
            ControllerButton::DPadLeft,
            ControllerButton::DPadDown,
            ControllerButton::DPadUp,
            ControllerButton::Start,
            ControllerButton::Z,
            ControllerButton::B,
            ControllerButton::A,
            ControllerButton::CRight,
            ControllerButton::CLeft,
            ControllerButton::CDown,
            ControllerButton::CUp,
            ControllerButton::TriggerRight,
            ControllerButton::TriggerLeft,
            ControllerButton::Reserved01,
            ControllerButton::Reserved02,
        ];

        buttons
            .into_iter()
            .filter(|&button| self.is_set(button))
            .collect()
    }
}
