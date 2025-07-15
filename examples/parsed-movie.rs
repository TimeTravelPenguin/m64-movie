use m64_movie::{BinReadExt, parsed::m64::Movie};

static MOVIE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/movies/1key.m64"));

fn main() {
    let bytes = MOVIE_BYTES.to_vec();
    let movie = Movie::from_bytes(&bytes).expect("Failed to parse movie bytes");

    println!("Author: {}", movie.recording_info.author_name);
    println!("Description: {}", movie.recording_info.description);
    println!("ROM: {}", movie.game_info.rom_name);
    println!("Start Type: {:?}", movie.recording_info.start_type);
    println!("Video Plugin: {}", movie.plugin_info.video_plugin);
    println!("Sound Plugin: {}", movie.plugin_info.sound_plugin);
    println!("Input Plugin: {}", movie.plugin_info.input_plugin);
    println!("RSP Plugin: {}", movie.plugin_info.rsp_plugin);
    println!(
        "Controller flags: {:#?}",
        movie.recording_info.controller_flags
    );

    // Find a nice starting point for the frames.
    let first_frame = movie
        .inputs
        .iter()
        .position(|input| input.a_btn() && input.b_btn() && input.axis() != (0, 0))
        .unwrap_or(0);

    let controller_inputs = movie
        .controller_inputs_stream()
        .enumerate()
        .skip(first_frame)
        .take(10);

    println!("\nFirst 10 frames from frame {}:", first_frame);
    for (i, input) in controller_inputs {
        println!("\nFrame {}:", i);

        for (j, controller) in input.enumerate() {
            println!(
                "  Controller {}: {:?} {:?}",
                j,
                controller.get_pressed(),
                controller.axis()
            );
        }
    }
}
