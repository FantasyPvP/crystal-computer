
use std::env;
use std::fs;

struct Filetypes {
	filename: String,
	source: u8,
	output: u8,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if &args[1] == "--help" {
		help();
		return;
	}
	let filetypes = Filetypes::new(&args);
	let file = fs::read_to_string(&filetypes.filename).expect("unable to open file");
	let cwd = env::current_dir().unwrap();
	let cwd = String::from(format!("{:?}", cwd));
	let cwd = &cwd[1..cwd.len()-1];
	fs::create_dir("./compiled-code");
	fs::create_dir("./compiled-code/assembly");

	compile_to_assembly(&filetypes: Filetypes, file: String);

	println!("\nsuccessfully compiled:\n    {}\ninto: \n    {}/compiled-code/assembly/ (step 1 of {})", filetypes.filename, cwd, &filetypes.output);
	if filetypes.output == 1 {
		println!("Done! \n - check './compiled-code/assembly/' from your current directory");
		return;
	}

	compile_to_binary()
}




impl Filetypes {
	fn new(args: &[String]) -> Filetypes {
		let filename = &args[1];
		let source_type = match &filename[filename.chars().count()-4..] {
			".ecy" => (1, format!("'{}' (basic crystal code)", filename)),
			".xcy" => (2, format!("'{}' (advanced crystal code)", filename)),
			_ => panic!("invalid file type (run with '--help' ?)")
		};
		
		let compile_type = match &args[2] as &str {
			"binary" => (2, format!("'{}.cby' (binary)", &filename[..filename.chars().count()-4])),
			"assembly" => (1, format!("'{}.cay' (assembly)", &filename[..filename.chars().count()-4])),
			_ => panic!("compile type must be 'assembly' or 'binary' (run with '--help' ?)")
		};

		println!("compiling file:\n    {} \ninto file:\n    {}", &source_type.1, &compile_type.1);
		let source = source_type.0 as u8;
		let output = compile_type.0 as u8;
		Filetypes { filename: filename.to_string(), source: source, output: output }
	}
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





fn tokenise_ecy() {
	unimplemented!()
}

fn tokenise_xcy() {
	unimplemented!()
}

fn tokenise_cay() {
	unimplemented!()
}

fn compile_ecy() {
	unimplemented!()
}

fn compile_xcy() {
	unimplemented!()
}

fn compile_cay() {
	unimplemented!()
}










