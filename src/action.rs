use std::process::Command;

pub fn copier(text: &str) {
    let output = Command::new("wl-copy")
        .arg(text)
        .output()
        .expect("failed to execute process");

    println!("{:?}", output);
}
