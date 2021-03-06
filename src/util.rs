use std::fs::remove_file;
use std::process::{self, Command, Output};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];

fn temp_file() -> String {
    format!("./temp_{}", process::id())
}

pub fn compile_test_cmd(filename: &str) -> Output {
    Command::new("rustc")
        .args(&["--test", filename, "-o", &temp_file()])
        .args(RUSTC_COLOR_ARGS)
        .output()
        .expect("failed to compile exercise")
}

pub fn compile_cmd(filename: &str) -> Output {
    Command::new("rustc")
        .args(&[filename, "-o", &temp_file()])
        .args(RUSTC_COLOR_ARGS)
        .output()
        .expect("failed to compile exercise")
}

pub fn run_cmd() -> Output {
    Command::new(&temp_file())
        .output()
        .expect("failed to run exercise")
}

pub fn clean() {
    let _ignored = remove_file(&temp_file());
}

#[test]
fn test_clean() {
    std::fs::File::create(&temp_file()).unwrap();
    clean();
    assert!(!std::path::Path::new(&temp_file()).exists());
}
