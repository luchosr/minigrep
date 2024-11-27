pub mod my_lib {
    use std::fs;

    pub fn run(file_path: &str) -> String {
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        contents
    }
}
