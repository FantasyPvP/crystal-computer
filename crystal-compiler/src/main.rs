
use std::env;
use std::fs;
pub mod lib;


fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	if filename == "--help" {
		help()
	};

	match &filename[filename.chars().count()-4..] {
		".ecy" => lib::init_ecy(filename, &args[2] as &str),
		".xcy" => lib::init_xcy(filename, &args[2] as &str),
		".xcy" => lib::init_cay(filename),
		".bcy" => panic!("why are you trying to compile a binary smh"),
		_ => panic!("invalid file type (run with '--help' ?)"),
	};
}



fn help() {
	println!("

[--help]

command line compiler utility for the FantasyPvP Crystal programming language
please use by running the compiler in the format:

'crystal-compiler filename output-type'

'output-type' refers to the type of file you want to recieve as an output, options:
	'assembly' (crystal assembly language)
	'binary' (crystal binary language that will run natively on a crystal vm)
	")
}
