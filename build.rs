use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("{:?}", out_dir);

    Command::new("cp").arg(&"/Users/mhawkins/workspace/libtcod/terminal.png").arg(&format!("{}/../../../../../", out_dir)).status().unwrap();
}