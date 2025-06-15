use std::process::Command;

fn main() {
    let output = Command::new("python")
        .arg("scraper.py")
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
