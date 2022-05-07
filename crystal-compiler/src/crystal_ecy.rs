pub fin init_ecy(filename: &str, compile_type: &str) {
	match compile_type {
		"assembly" => ()
		"binary" => let binary = init_acy(filename);
	}
}
