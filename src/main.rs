use clap::clap_app;
use png_rs::{decoder::*, encoder::*};

use std::convert::TryInto;


fn main() -> Result<(), DecoderError> {
	let matches = clap_app!(purgr =>
		(version: "0.3.1")
		(author: "Thatguyjs")
		(about: "Purge unnecessary tags from PNG images")
		(@arg INPUT_PATH: * -i --input +takes_value "Specifies the input PNG file")
		(@arg OUTPUT_PATH: * -o --output +takes_value "Specifies the output PNG file")
		(@arg ALLOWED_CHUNKS: -a --allow ... +takes_value "Allows specific chunk types to be copied to the output file")
	).get_matches();

	let mut allowed_names = vec![
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

	match matches.values_of("ALLOWED_CHUNKS") {
		Some(vals) => {
			for v in vals {
				if v.len() != 4 {
					println!("Invalid chunk name: {}", v);
					continue;
				}

				let tag_bytes = v.as_bytes().try_into().unwrap();
				if allowed_names.contains(&tag_bytes) { continue; }

				allowed_names.push(tag_bytes);
			}
		},
		None => {}
	}

	let input_path = matches.value_of("INPUT_PATH").unwrap();
	let output_path = matches.value_of("OUTPUT_PATH").unwrap();

	let mut decoder = ImageDecoder::open(input_path)?;
	let mut encoder = ImageEncoder::open(output_path).expect("Failed to create ImageEncoder");

	let mut chunks_removed = 0usize;

	for chunk in decoder.chunks() {
		match &chunk {
			Ok(c) => {
				if allowed_names.contains(&&c.ch_type) {
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
