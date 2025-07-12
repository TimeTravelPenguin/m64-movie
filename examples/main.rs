use m64_movie::Movie;

static MOVIE_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/movies/120 star tas (2012).m64"
));

static P2_MOVIE_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/movies/2c.m64"));

fn replace_bytes(bytes: &mut [u8], offset: usize, new_bytes: &[u8]) -> Result<(), String> {
    if offset + new_bytes.len() > bytes.len() {
        return Err("Replacement exceeds byte array length".to_string());
    }

    bytes[offset..offset + new_bytes.len()].copy_from_slice(new_bytes);
    Ok(())
}

fn main() {
    let bytes = P2_MOVIE_BYTES.to_vec();
    // replace_bytes(&mut bytes, 0x016, &[2])?;
    // replace_bytes(&mut bytes, 0x017, &[1])?;

    let movie = Movie::from_bytes(&bytes).expect("Failed to parse movie bytes");

    println!("Author: {:?}", movie.author_name);
    println!("Start Type: {:?}", movie.start_type);
    println!("ROM: {}", movie.rom_name);
    println!("Video Plugin: {}", movie.video_plugin);
    println!("Sound Plugin: {}", movie.sound_plugin);
    println!("Input Plugin: {}", movie.input_plugin);
    println!("RSP Plugin: {}", movie.rsp_plugin);
    println!("Extended Version: {}", movie.extended_version);
    println!("Extended Flags: {:?}", movie.extended_flags);
    println!("Controller flags: {:#?}", movie.controller_flags);

    let movie_bytes = movie.to_bytes().expect("Failed to serialize movie");

    println!("Original movie size: {}", MOVIE_BYTES.len());
    println!("Serialized movie size: {}", movie_bytes.len());
    println!("Equal: {}", MOVIE_BYTES == movie_bytes);

    // Offset of first difference
    let first_diff = MOVIE_BYTES
        .iter()
        .zip(movie_bytes.iter())
        .position(|(a, b)| a != b)
        .map(|pos| pos as i32)
        .unwrap_or(-1);

    println!("First difference at byte: {:#x}\n\n", first_diff);

    for controller in movie.inputs_grouped().iter() {
        for (i, input) in controller.iter().enumerate() {
            println!("Controller {}: {:?}", i, input.get_pressed());
        }
    }
}
