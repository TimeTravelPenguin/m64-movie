use std::io::Cursor;

use binrw::{BinWrite, meta::WriteEndian};
use m64_movie::{
    BinReadExt, BinWriteExt,
    movie::ControllerButton,
    parsed::m64::Movie,
    raw::m64::{
        ControllerFlags, ControllerState, ExtendedData, ExtendedFlags, MovieStartType, RawMovie,
    },
    shared::Reserved,
};

static MOVIE_120STAR_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/movies/120 star tas (2012).m64"
);

static MOVIE_120STAR_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/movies/120 star tas (2012).m64"
));

static MOVIE_1KEY_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/movies/1key.m64");

static MOVIE_1KEY_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/movies/1key.m64"));

/// Replaces a section of a byte slice with new bytes at the specified offset.
fn replace_bytes(bytes: &mut [u8], offset: usize, new_bytes: &[u8]) -> Result<(), String> {
    if offset + new_bytes.len() > bytes.len() {
        return Err("Replacement exceeds byte slice length".to_string());
    }

    bytes[offset..offset + new_bytes.len()].copy_from_slice(new_bytes);
    Ok(())
}

/// Asserts that the bytes of a BinWrite type match the expected byte slice.
fn assert_bytes_equal<T: BinWrite + WriteEndian>(actual: &T, expected: &[u8])
where
    for<'a> <T as BinWrite>::Args<'a>: std::default::Default,
{
    let mut cursor = Cursor::new(Vec::new());
    actual.write(&mut cursor).expect("Failed to write bytes");

    let actual_bytes = cursor.into_inner();
    assert!(actual_bytes.eq(expected), "Bytes do not match");
}

#[test]
fn test_parsed_movie_has_same_bytes() {
    for bytes in [MOVIE_120STAR_BYTES, MOVIE_1KEY_BYTES] {
        let raw_movie = RawMovie::from_bytes(bytes).unwrap();
        let movie_bytes = raw_movie.to_bytes().unwrap();

        assert!(
            bytes.eq(&movie_bytes),
            "Parsed movie bytes should match original"
        );
    }
}

#[test]
fn test_movie_parsing_basic_properties() {
    // NOTE: This test assumes that the movies are valid and well-formed.
    // This should be updated when additional movies are added for testing.
    for bytes in [MOVIE_120STAR_BYTES, MOVIE_1KEY_BYTES] {
        let movie = RawMovie::from_bytes(bytes).unwrap();

        // Check basic properties
        assert_eq!(movie.version, 3);
        assert_eq!(movie.controller_count, 1);
        assert_ne!(movie.uid, 0);
        assert!(movie.vis_per_second > 0);
        assert!(!movie.inputs.is_empty());
    }
}

#[test]
fn test_movie_inputs_grouped() {
    let movie = RawMovie::from_bytes(MOVIE_120STAR_BYTES).unwrap();
    let grouped = movie.controller_inputs_stream().collect::<Vec<_>>();

    // Should have the correct number of groups
    assert_eq!(
        grouped.len(),
        movie.inputs.len() / movie.controller_count as usize
    );

    // Each group should have the correct number of controllers
    for group in grouped {
        assert_eq!(group.count(), movie.controller_count as usize);
    }
}

#[test]
fn test_button_state_to_bytes() {
    // DPad makes up the first 4 bits:
    let mut btn = ControllerState::default();
    btn.set_dpad_up(true);
    btn.set_dpad_down(true);
    btn.set_dpad_left(true);
    btn.set_dpad_right(true);

    let bytes = u32::from(btn).to_le_bytes();
    assert_eq!(
        bytes,
        [15, 0, 0, 0],
        "ButtonState should serialize to [15, 0, 0, 0]"
    );
}

#[test]
fn test_button_state_from_bytes() {
    // DPad makes up the first 4 bits:
    let bytes: u32 = u32::from_le_bytes([15, 0, 0, 0]);
    let button_state = ControllerState::from(bytes);

    assert!(button_state.dpad_right(), "DPad Right should be pressed");
    assert!(button_state.dpad_left(), "DPad Left should be pressed");
    assert!(button_state.dpad_down(), "DPad Down should be pressed");
    assert!(button_state.dpad_up(), "DPad Up should be pressed");
}

#[test]
fn test_controller_state_default() {
    let state = ControllerState::default();

    // All buttons should be unpressed by default
    assert!(!state.dpad_right());
    assert!(!state.dpad_left());
    assert!(!state.dpad_down());
    assert!(!state.dpad_up());
    assert!(!state.start_btn());
    assert!(!state.z_btn());
    assert!(!state.b_btn());
    assert!(!state.a_btn());
    assert!(!state.c_right());
    assert!(!state.c_left());
    assert!(!state.c_down());
    assert!(!state.c_up());
    assert!(!state.trigger_right());
    assert!(!state.trigger_left());
    assert!(!state.reserved01());
    assert!(!state.reserved02());

    // Axes should be centered
    assert_eq!(state.x_axis(), 0);
    assert_eq!(state.y_axis(), 0);
}

#[test]
fn test_controller_state_button_operations() {
    let mut state = ControllerState::default();

    // Test setting buttons
    state.set(ControllerButton::A);
    assert!(state.is_set(ControllerButton::A));
    assert!(!state.is_set(ControllerButton::B));

    state.set(ControllerButton::B);
    assert!(state.is_set(ControllerButton::A));
    assert!(state.is_set(ControllerButton::B));

    // Test unsetting buttons
    state.unset(ControllerButton::A);
    assert!(!state.is_set(ControllerButton::A));
    assert!(state.is_set(ControllerButton::B));

    // Test toggling buttons
    state.toggle(ControllerButton::Start);
    assert!(state.is_set(ControllerButton::Start));
    state.toggle(ControllerButton::Start);
    assert!(!state.is_set(ControllerButton::Start));
}

#[test]
fn test_controller_state_axis_operations() {
    let mut state = ControllerState::default();

    // Test setting individual axes
    state.set_x_axis(127);
    state.set_y_axis(-128);

    assert_eq!(state.x_axis(), 127);
    assert_eq!(state.y_axis(), -128);

    // Test setting both axes at once
    state.set_axis(-64, 64);
    assert_eq!(state.axis(), (-64, 64));
}

#[test]
fn test_controller_state_get_pressed() {
    let mut state = ControllerState::default();

    // Set some buttons
    state.set(ControllerButton::A);
    state.set(ControllerButton::B);
    state.set(ControllerButton::Start);
    state.set(ControllerButton::DPadUp);

    let pressed = state.get_pressed();

    assert!(pressed.contains(&ControllerButton::A));
    assert!(pressed.contains(&ControllerButton::B));
    assert!(pressed.contains(&ControllerButton::Start));
    assert!(pressed.contains(&ControllerButton::DPadUp));
    assert!(!pressed.contains(&ControllerButton::Z));
}

#[test]
fn test_extended_flags() {
    let mut flags = ExtendedFlags::default();

    // Test default state
    assert!(!flags.wiivc_emulation_mode());

    // Test setting WiiVC emulation mode
    flags.set_wiivc_emulation_mode(true);
    assert!(flags.wiivc_emulation_mode());

    flags.set_wiivc_emulation_mode(false);
    assert!(!flags.wiivc_emulation_mode());
}

#[test]
fn test_controller_flags() {
    let mut flags = ControllerFlags::default();

    // Test default state
    assert!(!flags.controller_01_present());
    assert!(!flags.controller_02_present());
    assert!(!flags.controller_03_present());
    assert!(!flags.controller_04_present());

    // Test setting controllers
    flags.set_controller_01_present(true);
    flags.set_controller_02_present(true);

    assert!(flags.controller_01_present());
    assert!(flags.controller_02_present());
    assert!(!flags.controller_03_present());
    assert!(!flags.controller_04_present());

    // Test controller count
    assert_eq!(flags.num_controllers_present(), 2);

    // Test memory pack flags
    flags.set_controller_01_has_mempak(true);
    assert!(flags.controller_01_has_mempak());

    // Test rumble pack flags
    flags.set_controller_02_has_rumblepak(true);
    assert!(flags.controller_02_has_rumblepak());
}

#[test]
fn test_movie_start_types() {
    let pairs = [
        (MovieStartType::Snapshot, 1u16),
        (MovieStartType::PowerOn, 2u16),
        (MovieStartType::EEPROM, 4u16),
    ];

    for (start_type, expected_value) in pairs.iter() {
        assert_bytes_equal(start_type, &expected_value.to_le_bytes());
    }
}

#[test]
fn test_extended_data() {
    let mut data = ExtendedData {
        authorship_info: 0,
        bruteforce_data: 0,
        rerecord_count_high: 0,
        reserved: Reserved { reserved: [0; 20] },
    };

    // Test default values
    assert_eq!(data.authorship_info, 0);
    assert_eq!(data.bruteforce_data, 0);
    assert_eq!(data.rerecord_count_high, 0);

    // Test setting values
    data.authorship_info = 12345;
    data.bruteforce_data = 67890;
    data.rerecord_count_high = 11111;

    assert_eq!(data.authorship_info, 12345);
    assert_eq!(data.bruteforce_data, 67890);
    assert_eq!(data.rerecord_count_high, 11111);
}

#[test]
fn test_movie_rom_info() {
    let movie = RawMovie::from_bytes(MOVIE_120STAR_BYTES).unwrap();

    // ROM name should be ASCII and not empty
    let rom_name = movie.rom_name.to_string();
    assert!(!rom_name.is_empty());
    assert!(rom_name.is_ascii());

    // CRC32 should be non-zero
    assert_ne!(movie.rom_crc32, 0);
    assert!(movie.rom_country > 0);
}

#[test]
fn test_movie_plugin_names() {
    let movie = RawMovie::from_bytes(MOVIE_120STAR_BYTES).unwrap();

    // All plugin names should be ASCII
    assert!(movie.video_plugin.to_string().is_ascii());
    assert!(movie.sound_plugin.to_string().is_ascii());
    assert!(movie.input_plugin.to_string().is_ascii());
    assert!(movie.rsp_plugin.to_string().is_ascii());
}

#[test]
fn test_movie_author_info() {
    let movie = RawMovie::from_bytes(MOVIE_120STAR_BYTES).unwrap();

    // Author name and description should be ASCII
    assert!(movie.author_name.to_string().is_ascii());
    assert!(movie.description.to_string().is_ascii());
}

#[test]
fn test_invalid_magic() {
    let mut invalid_bytes = MOVIE_120STAR_BYTES.to_vec();

    // Change the magic bytes
    invalid_bytes[0..4].copy_from_slice(b"INV\x1A");

    let result = RawMovie::from_bytes(&invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_invalid_version() {
    let mut invalid_bytes = MOVIE_120STAR_BYTES.to_vec();

    // Change version to something other than 3
    invalid_bytes[4..8].copy_from_slice(&2u32.to_le_bytes());

    let result = RawMovie::from_bytes(&invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_movie_roundtrip() {
    let original_movie = RawMovie::from_bytes(MOVIE_120STAR_BYTES).unwrap();
    let movie_bytes = original_movie.to_bytes().unwrap();
    let roundtrip_movie = RawMovie::from_bytes(&movie_bytes).unwrap();

    // The movies should be identical
    assert_eq!(original_movie, roundtrip_movie);
}

#[test]
fn test_to_file_matches_input_file() {
    let input_files = [MOVIE_120STAR_PATH, MOVIE_1KEY_PATH];

    for &file_path in &input_files {
        let movie = RawMovie::from_file(file_path).unwrap();

        let temp_file = tempfile::NamedTempFile::new().unwrap();
        movie.to_file(temp_file.path()).unwrap();

        let written_bytes = std::fs::read(temp_file.path()).unwrap();
        let original_bytes = std::fs::read(file_path).unwrap();

        assert_eq!(
            written_bytes, original_bytes,
            "Written bytes should match original file"
        );
    }
}

#[test]
fn test_ext_version_cases() -> Result<(), String> {
    let mut bytes = MOVIE_120STAR_BYTES.to_vec();

    let movie =
        RawMovie::from_bytes(&bytes).map_err(|e| format!("Failed to parse movie: {}", e))?;
    assert_eq!(movie.extended_version, 0, "Initial ext_version should be 0");

    // Set the extended version to 1
    replace_bytes(
        &mut bytes,
        0x16, // Offset for extended version
        &1u8.to_le_bytes(),
    )?;

    let movie =
        RawMovie::from_bytes(&bytes).map_err(|e| format!("Failed to parse movie: {}", e))?;
    assert_eq!(movie.extended_version, 1, "Extended version should be 1");

    // Set the extended flags to enable WiiVC emulation mode.
    // This should be valid for ext_version 1.
    let mut ext_flags = ExtendedFlags::default();
    ext_flags.set_wiivc_emulation_mode(true);

    let ext_flags_bytes = ext_flags
        .to_bytes()
        .map_err(|e| format!("Failed to serialize ExtendedFlags: {}", e))?;

    replace_bytes(
        &mut bytes,
        0x17, // Offset for extended flags
        &ext_flags_bytes,
    )?;

    let movie =
        RawMovie::from_bytes(&bytes).map_err(|e| format!("Failed to parse movie: {}", e))?;
    assert_eq!(
        movie.extended_flags, ext_flags,
        "Extended flags should match after modification"
    );

    // Now set the extended version to 0, which should not be valid.
    replace_bytes(
        &mut bytes,
        0x16, // Offset for extended version
        &0u8.to_le_bytes(),
    )?;

    let movie = RawMovie::from_bytes(&bytes);
    assert!(
        movie.is_err(),
        "RawMovie should not parse with ext_version 1"
    );

    Ok(())
}

#[test]
fn test_parsed_from_raw_movie() {
    let raw_movie = RawMovie::from_bytes(MOVIE_120STAR_BYTES).unwrap();
    let parsed_movie: Movie = raw_movie.clone().try_into().unwrap();

    // Metadata checks
    assert_eq!(parsed_movie.metadata.version, raw_movie.version);
    assert_eq!(
        parsed_movie.metadata.extended_version,
        raw_movie.extended_version
    );
    assert_eq!(
        parsed_movie.metadata.extended_flags,
        raw_movie.extended_flags
    );
    assert_eq!(parsed_movie.metadata.extended_data, raw_movie.extended_data);

    // Game info checks
    assert_eq!(
        parsed_movie.game_info.rom_name.to_string(),
        raw_movie.rom_name.to_string()
    );
    assert_eq!(parsed_movie.game_info.rom_crc32, raw_movie.rom_crc32);
    assert_eq!(parsed_movie.game_info.rom_country, raw_movie.rom_country);

    // Plugin info checks
    assert_eq!(
        parsed_movie.plugin_info.video_plugin.to_string(),
        raw_movie.video_plugin.to_string()
    );
    assert_eq!(
        parsed_movie.plugin_info.sound_plugin.to_string(),
        raw_movie.sound_plugin.to_string()
    );
    assert_eq!(
        parsed_movie.plugin_info.input_plugin.to_string(),
        raw_movie.input_plugin.to_string()
    );
    assert_eq!(
        parsed_movie.plugin_info.rsp_plugin.to_string(),
        raw_movie.rsp_plugin.to_string()
    );

    // Recording info checks
    assert_eq!(
        parsed_movie.recording_info.author_name.to_string(),
        raw_movie.author_name.to_string()
    );
    assert_eq!(
        parsed_movie.recording_info.description.to_string(),
        raw_movie.description.to_string()
    );
    assert_eq!(parsed_movie.recording_info.uid, raw_movie.uid);
    assert_eq!(
        parsed_movie.recording_info.vertical_interrupts,
        raw_movie.vertical_interrupts
    );
    assert_eq!(
        parsed_movie.recording_info.rerecord_count,
        raw_movie.rerecord_count
    );
    assert_eq!(
        parsed_movie.recording_info.vis_per_second,
        raw_movie.vis_per_second
    );
    assert_eq!(
        parsed_movie.recording_info.controller_count,
        raw_movie.controller_count
    );
    assert_eq!(
        parsed_movie.recording_info.controller_input_samples,
        raw_movie.controller_input_samples
    );
    assert_eq!(
        parsed_movie.recording_info.controller_flags,
        raw_movie.controller_flags
    );
    assert_eq!(parsed_movie.recording_info.start_type, raw_movie.start_type);

    // Inputs checks
    assert_eq!(parsed_movie.inputs, raw_movie.inputs);
}
