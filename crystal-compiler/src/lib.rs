






pub fn init_ecy(filename: &str, compile_type: &str) {








	match compile_type {
        "assembly" => ()
        "binary" => let binary = init_cay(filename);
    }
}





pub fn init_xcy(filename: &str, compile_type: &str) {






    match compile_type {
        "assembly" => ()
        "binary" => let binary = init_cay(filename);
    }
}






pub fn init_acy(filename: &str) {

    

    let mut output = File::create(filename)?;
    write!(output, binary_str)?;
}
