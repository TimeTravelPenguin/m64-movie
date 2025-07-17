use crate::{
    MovieError, MovieParseError,
    raw::{self, ControllerFlags, ControllerState, MovieStartType, RawMovie},
    shared::{Ascii, EncodedFixedStr, Reserved, Utf8},
};

/// Extended flags for Mupen64 movies.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExtendedFlags {
    ExtendedFlagsV0,
    ExtendedFlagsV1 { wiivc_emulation_mode: bool },
}

impl From<ExtendedFlags> for raw::ExtendedFlags {
    fn from(flags: ExtendedFlags) -> raw::ExtendedFlags {
        let mut raw_flags = raw::ExtendedFlags::default();
        match flags {
            ExtendedFlags::ExtendedFlagsV0 => {}
            ExtendedFlags::ExtendedFlagsV1 {
                wiivc_emulation_mode,
            } => {
                raw_flags.set_wiivc_emulation_mode(wiivc_emulation_mode);
            }
        }

        raw_flags
    }
}

/// Extended data for Mupen64 movies.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExtendedData {
    ExtendedDataV0,
    ExtendedDataV1 {
        /// Special authorship information.
        authorship_info: u32,
        /// Data regarding bruteforcing.
        bruteforce_data: u32,
        /// The high word of the rerecord count.
        rerecord_count_high: u32,
    },
}

impl From<ExtendedData> for raw::ExtendedData {
    fn from(data: ExtendedData) -> raw::ExtendedData {
        match data {
            ExtendedData::ExtendedDataV0 => raw::ExtendedData::default(),
            ExtendedData::ExtendedDataV1 {
                authorship_info,
                bruteforce_data,
                rerecord_count_high,
            } => raw::ExtendedData {
                authorship_info,
                bruteforce_data,
                rerecord_count_high,
                reserved: Reserved::default(),
            },
        }
    }
}

/// Metadata for a Mupen64 movie file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MupenMetadata {
    /// The version of the Mupen64 movie format.
    pub version: u32,
    /// The extended version of the movie format. On versions of Mupen64 movies
    /// created with mupen <1.1.9, this value is always 0.
    pub extended_version: u8,
    /// Extended flags for the movie. This is only valid if the extended version is 1.
    pub extended_flags: ExtendedFlags,
    /// Extended data for the movie, which is only valid if the extended version is non-zero.
    pub extended_data: ExtendedData,
}

/// Information about the game used in the movie.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameInfo {
    /// The internal name of the ROM used in the movie. This value is taken
    /// directly from the ROM. Should be a 32-byte ASCII string.
    pub rom_name: EncodedFixedStr<32, Ascii>,
    /// The CRC32 checksum of the ROM used in the movie. This value is taken
    /// directly from the ROM.
    pub rom_crc32: u32,
    /// The country code of the ROM used in the movie. This value is taken
    /// directly from the ROM.
    pub rom_country: u16,
}

/// Information about the plugins used in the movie.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PluginInfo {
    /// The name of the video plugin used in the movie. This value is
    /// taken directly from the plugin. Should be a 64-byte ASCII string.
    pub video_plugin: EncodedFixedStr<64, Ascii>,
    /// The name of the sound plugin used in the movie. This value is
    /// taken directly from the plugin. Should be 64-byte ASCII string.
    pub sound_plugin: EncodedFixedStr<64, Ascii>,
    /// The name of the input plugin used in the movie. This value is
    /// taken directly from the plugin. Should be 64-byte ASCII string.
    pub input_plugin: EncodedFixedStr<64, Ascii>,
    /// The name of the RSP plugin used in the movie. This value is
    /// taken directly from the plugin. Should be 64-byte ASCII string.
    pub rsp_plugin: EncodedFixedStr<64, Ascii>,
}

/// Information about the recording, including author and movie details.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RecordingInfo {
    /// Author name info for the movie. Should be 222-byte UTF-8 string.
    pub author_name: EncodedFixedStr<222, Utf8>,
    /// Author description info for the movie. Should be 256-byte UTF-8 string.
    pub description: EncodedFixedStr<256, Utf8>,
    /// The unique identifier for the movie.
    pub uid: u32,
    /// The number of vertical interrupts in the movie.
    pub vertical_interrupts: u32,
    /// The number of rerecords in the movie.
    pub rerecord_count: u32,
    /// The number of vertical interrupts per second.
    pub vis_per_second: u8,
    /// The number of controllers used in the movie.
    pub controller_count: u8,
    /// The number of input samples for any controller in the movie.
    pub controller_input_samples: u32,
    /// Flags indicating the presence and capabilities of controllers.
    pub controller_flags: ControllerFlags,
    /// The start type of the movie, indicating how the movie begins.
    pub start_type: MovieStartType,
}

/// A Mupen64 movie file.
///
/// Only version 3 is supported. Please refer to the
/// [file format documentation](https://tasvideos.org/EmulatorResources/Mupen/M64) for more details.

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Movie {
    /// Metadata about the Mupen64 movie format.
    pub metadata: MupenMetadata,
    /// Information about the game used in the movie.
    pub game_info: GameInfo,
    /// Information about the plugins used in the movie.
    pub plugin_info: PluginInfo,
    /// Information about the recording, including author and movie details.
    pub recording_info: RecordingInfo,
    /// Controller inputs for the movie.
    pub inputs: Vec<ControllerState>,
}

pub trait MovieDetails {
    /// Creates a new instance from the raw type.
    fn from_raw(raw: &RawMovie) -> Result<Self, MovieError>
    where
        Self: Sized;
}

impl MovieDetails for ExtendedFlags {
    fn from_raw(raw: &RawMovie) -> Result<Self, MovieError> {
        if raw.extended_version == 0 {
            Ok(ExtendedFlags::ExtendedFlagsV0)
        } else if raw.extended_version == 1 {
            Ok(ExtendedFlags::ExtendedFlagsV1 {
                wiivc_emulation_mode: raw.extended_flags.wiivc_emulation_mode(),
            })
        } else {
            Err(MovieParseError::UnsupportedExtendedVersion(raw.extended_version).into())
        }
    }
}

impl MovieDetails for ExtendedData {
    fn from_raw(raw: &RawMovie) -> Result<Self, MovieError> {
        if raw.extended_version == 0 {
            Ok(ExtendedData::ExtendedDataV0)
        } else if raw.extended_version == 1 {
            Ok(ExtendedData::ExtendedDataV1 {
                authorship_info: raw.extended_data.authorship_info,
                bruteforce_data: raw.extended_data.bruteforce_data,
                rerecord_count_high: raw.extended_data.rerecord_count_high,
            })
        } else {
            Err(MovieParseError::UnsupportedExtendedVersion(raw.extended_version).into())
        }
    }
}

impl MovieDetails for MupenMetadata {
    fn from_raw(raw: &RawMovie) -> Result<Self, MovieError> {
        if raw.version != 3 {
            return Err(MovieParseError::UnsupportedVersion(raw.version).into());
        }

        let extended_flags = ExtendedFlags::from_raw(raw)?;
        let extended_data = ExtendedData::from_raw(raw)?;

        Ok(MupenMetadata {
            version: raw.version,
            extended_version: raw.extended_version,
            extended_flags,
            extended_data,
        })
    }
}

impl MovieDetails for GameInfo {
    fn from_raw(raw: &RawMovie) -> Result<Self, MovieError> {
        Ok(GameInfo {
            rom_name: EncodedFixedStr::from_ascii_str(raw.rom_name.to_string())?,
            rom_crc32: raw.rom_crc32,
            rom_country: raw.rom_country,
        })
    }
}

impl MovieDetails for PluginInfo {
    fn from_raw(raw: &RawMovie) -> Result<Self, MovieError> {
        Ok(PluginInfo {
            video_plugin: EncodedFixedStr::from_ascii_str(raw.video_plugin.to_string())?,
            sound_plugin: EncodedFixedStr::from_ascii_str(raw.sound_plugin.to_string())?,
            input_plugin: EncodedFixedStr::from_ascii_str(raw.input_plugin.to_string())?,
            rsp_plugin: EncodedFixedStr::from_ascii_str(raw.rsp_plugin.to_string())?,
        })
    }
}

impl MovieDetails for RecordingInfo {
    fn from_raw(raw: &RawMovie) -> Result<Self, MovieError> {
        Ok(RecordingInfo {
            author_name: EncodedFixedStr::from_utf8_str(raw.author_name.to_string())?,
            description: EncodedFixedStr::from_utf8_str(raw.description.to_string())?,
            uid: raw.uid,
            vertical_interrupts: raw.vertical_interrupts,
            rerecord_count: raw.rerecord_count,
            vis_per_second: raw.vis_per_second,
            controller_count: raw.controller_count,
            controller_input_samples: raw.controller_input_samples,
            controller_flags: raw.controller_flags,
            start_type: raw.start_type,
        })
    }
}

impl TryFrom<RawMovie> for Movie {
    type Error = MovieError;

    fn try_from(raw: RawMovie) -> Result<Self, Self::Error> {
        Ok(Movie {
            metadata: MupenMetadata::from_raw(&raw)?,
            game_info: GameInfo::from_raw(&raw)?,
            plugin_info: PluginInfo::from_raw(&raw)?,
            recording_info: RecordingInfo::from_raw(&raw)?,
            inputs: raw.inputs,
        })
    }
}

impl From<Movie> for RawMovie {
    fn from(movie: Movie) -> Self {
        RawMovie {
            version: movie.metadata.version,
            extended_version: movie.metadata.extended_version,
            extended_flags: movie.metadata.extended_flags.into(),
            extended_data: movie.metadata.extended_data.into(),
            rom_name: movie.game_info.rom_name.to_string().into(),
            rom_crc32: movie.game_info.rom_crc32,
            rom_country: movie.game_info.rom_country,
            video_plugin: movie.plugin_info.video_plugin.to_string().into(),
            sound_plugin: movie.plugin_info.sound_plugin.to_string().into(),
            input_plugin: movie.plugin_info.input_plugin.to_string().into(),
            rsp_plugin: movie.plugin_info.rsp_plugin.to_string().into(),
            author_name: movie.recording_info.author_name.to_string().into(),
            description: movie.recording_info.description.to_string().into(),
            uid: movie.recording_info.uid,
            vertical_interrupts: movie.recording_info.vertical_interrupts,
            rerecord_count: movie.recording_info.rerecord_count,
            vis_per_second: movie.recording_info.vis_per_second,
            controller_count: movie.recording_info.controller_count,
            controller_input_samples: movie.recording_info.controller_input_samples,
            controller_flags: movie.recording_info.controller_flags,
            start_type: movie.recording_info.start_type,
            inputs: movie.inputs,
            reserved01: Reserved::default(),
            reserved02: Reserved::default(),
            reserved03: Reserved::default(),
        }
    }
}

impl Movie {
    /// Creates a new [`Movie`] from a [`RawMovie`].
    pub fn from_raw(raw: RawMovie) -> Result<Self, MovieError> {
        Self::try_from(raw)
    }

    /// Converts the [`Movie`] into a [`RawMovie`].
    pub fn into_raw(self) -> RawMovie {
        RawMovie::from(self)
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
            .chunks(self.recording_info.controller_count as usize)
            .map(move |chunk| chunk.iter())
    }
}
