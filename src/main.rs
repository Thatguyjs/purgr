mod args;
use args::*;

use png_rs::{decoder::*, encoder::*};

fn main() -> Result<(), DecoderError> {
	let save_types = vec![
		b"IHDR", b"IEND",
		b"PLTE", b"IDAT",
		b"cHRM",
		b"gAMA",
		b"iCCP",
		b"sBIT",
		b"sRGB",
		b"bKGD",
		// b"hIST" <- Not required?
		b"tRNS",
		b"pHYs", // Make this optional
	];

	let env_args = Args::from_env();
	let input_cmd = Command::new("input".into(), "i".into(), "Specify the input filepath".into(), 1);
	let output_cmd = Command::new("output".into(), "o".into(), "Specify the output filepath".into(), 1);

	let input_path = input_cmd.get_from_args(&env_args).expect("A filepath input argument must be specified");
	let output_path = output_cmd.get_from_args(&env_args).expect("A filepath output argument must be specified");

	let mut decoder = ImageDecoder::open(&input_path[0])?;
	let mut encoder = ImageEncoder::open(&output_path[0]).expect("Failed to create ImageEncoder");

	let mut chunks_removed = 0usize;

	for chunk in decoder.chunks() {
		match &chunk {
			Ok(c) => {
				if save_types.contains(&&c.ch_type) {
					match encoder.write_chunk(c) {
						Ok(_) => { println!("Wrote chunk: {}", std::str::from_utf8(&c.ch_type).unwrap_or("[Invalid Type String]")); },
						Err(e) => { println!("Failed to write chunk: {}", e); }
					}
				}
				else {
					chunks_removed += 1;
				}
			},
			Err(e) => panic!("Failed to read image chunk: {}", e)
		}
	}

	println!("Done!\nChunks removed: {}", chunks_removed);
	Ok(())
}
