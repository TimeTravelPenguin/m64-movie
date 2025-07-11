use m64_movie::{ButtonState, Movie};

static MOVIE_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/movies/120 star tas (2012).m64"
));

fn replace_bytes(bytes: &mut [u8], offset: usize, new_bytes: &[u8]) -> Result<(), String> {
    if offset + new_bytes.len() > bytes.len() {
        return Err("Replacement exceeds byte slice length".to_string());
    }

    bytes[offset..offset + new_bytes.len()].copy_from_slice(new_bytes);
    Ok(())
}

#[test]
fn test_parsed_movie_has_same_bytes() {
    // TODO: Return to this test when complete.

    let raw_movie = Movie::from_bytes(MOVIE_BYTES).unwrap();
    let movie_bytes = raw_movie.to_bytes().unwrap();

    assert!(
        MOVIE_BYTES.eq(&movie_bytes),
        "Parsed movie bytes should match original"
    );
}

#[test]
fn test_button_state_to_bytes() {
    // DPad makes up the first 4 bits:
    let mut btn = ButtonState::default();
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
    let bytes: u32 = u32::from_le_bytes([15, 00, 00, 00]);
    let button_state = ButtonState::from(bytes);

    assert!(button_state.dpad_right(), "DPad Right should be pressed");
    assert!(button_state.dpad_left(), "DPad Left should be pressed");
    assert!(button_state.dpad_down(), "DPad Down should be pressed");
    assert!(button_state.dpad_up(), "DPad Up should be pressed");
}
