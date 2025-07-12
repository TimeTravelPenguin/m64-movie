use m64_movie::Movie;

static MOVIE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/movies/1key.m64"));

fn main() {
    let bytes = MOVIE_BYTES.to_vec();
    let movie = Movie::from_bytes(&bytes).expect("Failed to parse movie bytes");

    println!("Author: {}", movie.author_name);
    println!("Description: {}", movie.description);
    println!("ROM: {}", movie.rom_name);
    println!("Start Type: {:?}", movie.start_type);
    println!("Video Plugin: {}", movie.video_plugin);
    println!("Sound Plugin: {}", movie.sound_plugin);
    println!("Input Plugin: {}", movie.input_plugin);
    println!("RSP Plugin: {}", movie.rsp_plugin);
    println!("Controller flags: {:#?}", movie.controller_flags);

    // Find a nice starting point for the frames.
    let first_frame = movie
        .inputs
        .iter()
        .position(|input| input.a_btn() && input.b_btn() && input.axis() != (0, 0))
        .unwrap_or(0);

    println!("\nFirst 10 frames from frame {}:", first_frame);
    for (i, input) in movie.inputs.iter().enumerate().skip(first_frame).take(10) {
        println!("Frame {}: {:?} {:?}", i, input.get_pressed(), input.axis());
    }
}
