use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=frontend");

    Command::new("npm.cmd")
        .args(["run", "build", "--prefix", "frontend"])
        .output()
        .expect("Failed to build frontend.");
}