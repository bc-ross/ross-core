use std::{env, process::Command};

fn main() {
    let output = Command::new("python")
        .arg("scraper.py")
        .arg(env::var("CRATE_PROFILE").unwrap())
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
