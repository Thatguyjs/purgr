// A very simple CLI argument parser

use std::env;


pub struct Args {
	raw_args: Vec<String>
}

impl Args {
	pub fn from_env() -> Self {
		Self {
			raw_args: env::args().collect()
		}
	}

	pub fn parse_arg(&self, arg: &str, arg_count: usize) -> Option<Vec<String>> {
		let mut found = false;
		let mut args = vec![];

		for raw_arg in &self.raw_args {
			if found {
				if args.len() == arg_count { return Some(args); }
				else { args.push(raw_arg.clone()); }
			}

			if raw_arg == arg { found = true; }
		}

		if found { return Some(args); }
		None
	}
}


#[derive(Debug)]
pub struct Command {
	pub name: String,
	pub short_name: String,
	pub description: String,
	pub arg_count: usize
}

impl Command {
	pub fn new(name: String, short_name: String, description: String, arg_count: usize) -> Self {
		Self {
			name,
			short_name,
			description,
			arg_count
		}
	}

	pub fn get_from_args(&self, args: &Args) -> Option<Vec<String>> {
		if let Some(value) = args.parse_arg(&format!("--{}", &self.name), self.arg_count) {
			return Some(value);
		}
		else if self.short_name.len() > 0 {
			if let Some(value) = args.parse_arg(&format!("-{}", &self.short_name), self.arg_count) {
				return Some(value);
			}
		}

		None
	}
}


#[derive(Debug)]
pub struct Arg<'a>(&'a Command, Vec<String>);

pub struct CommandList {
	pub commands: Vec<Command>
}

impl CommandList {
	pub fn new() -> Self {
		Self { commands: vec![] }
	}

	pub fn from_commands(commands: Vec<Command>) -> Self {
		Self { commands }
	}

	pub fn add_command(&mut self, command: Command) {
		self.commands.push(command);
	}

	pub fn get_all_args(&self, args: &Args) -> Vec<Arg> {
		let mut result = vec![];

		for command in &self.commands {
			if let Some(value) = command.get_from_args(args) {
				result.push(Arg(command, value));
			}
		}

		result
	}
}
