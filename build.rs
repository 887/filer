use std::process::Command;

fn main() {
    // println!("compiling resources..");
    Command::new("glib-compile-resources")
        .args(&["--generate", "resources.xml"])
        .current_dir("data")
        .status()
        .unwrap();

    // println!("compiling schemas..");
    Command::new("glib-compile-schemas")
        .args(&["."])
        .current_dir("data")
        .status()
        .unwrap();
}
