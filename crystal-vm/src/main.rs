use std::fs;
use std::io;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct Sys {
	cir: u32, // current instruction register
	acc: u32, // accumulator
	ncr: u32, // next instruction register
	ram: Vec<u32>,
}

impl Sys {
	fn new(location: String) -> Sys { // creates a new instance of the CPU

		let fs_root = location;
	
		Sys {
			cir: 00000000, 
			acc: 00000000, 
			ncr: 00000000,
			ram: Vec::new(),
		}
	}
	fn add(&self, location_a: u32, location_b: u32) {
		match location_a[-4..]{
			
		}
	}
}


















fn main() {

	let args: Vec<String> = env::args().collect();
	let location = &args[0];

	let sys = Sys::new(location.to_string());

	println!("{:?}", sys)


	
}
