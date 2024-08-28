use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("../../../../assets/");
    if !dest_path.exists() {
        fs::create_dir_all(&dest_path).unwrap();
    }
    fs::copy("assets/target.png", dest_path.join("target.png")).unwrap();
    println!("cargo:rerun-if-changed=assets/target.png");
}
