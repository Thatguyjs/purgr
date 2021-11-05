mod args;
use args::*;

use png_rs::decoder::DecoderError;

fn main() -> Result<(), DecoderError> {
	let env_args = Args::from_env();
	let input_cmd = Command::new("input".into(), "i".into(), "Specify the input filepath".into(), 1);
	let output_cmd = Command::new("output".into(), "o".into(), "Specify the output filepath".into(), 1);

	let input_path = input_cmd.get_from_args(&env_args).expect("A filepath input argument must be specified");
	let output_path = output_cmd.get_from_args(&env_args).expect("A filepath output argument must be specified");

	let mut decoder = png_rs::decoder::ImageDecoder::open(&input_path[0])?;
	// let mut encoder = png_rs::encoder::ImageEncoder::open(&output_path[0])?;

	for chunk in decoder.chunks() {
		match &chunk {
			Ok(c) => {
				// if &c.ch_type == b"IDHR" || &c.ch_type == b"IEND" || &c.ch_type == b"PLTE" || &c.ch_type == b"IDAT" {
				// 	encoder.write_chunk(c);
				// }
			},
			Err(e) => panic!("Failed to read image chunk: {}", e)
		}
	}

	// encoder.close();
	Ok(())
}
