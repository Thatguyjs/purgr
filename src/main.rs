mod args;
use args::*;

use png_rs;

fn main() {
	let commands = CommandList::from_commands(vec![
		Command::new("help".into(), "h".into(), "Show a list of commands".into(), 0),
		Command::new("input".into(), "i".into(), "Specifies the input file".into(), 1)
	]);

	println!("{:?}", commands.get_all_args(&Args::from_env()));
}
