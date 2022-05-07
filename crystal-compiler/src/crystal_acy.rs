pub fn init_acy(filename: &str) {
	let mut output = File::create(filename)?;
	write!(output, binary_str)?;
}
