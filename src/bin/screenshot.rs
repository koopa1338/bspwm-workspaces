use std::{env, path::PathBuf, process::Command};

fn main() {
    let mut args = env::args();
    args.next();
    if let Some(mode) = args.next() {
        let mut path = PathBuf::new();
        path.push(env!("HOME"));
        path.push("Bilder/Screenshots");
        let flameshot_args = match mode.as_str() {
            "full" => {
                vec!["full", "-p", path.to_str().unwrap()]
            }
            "selection" => vec!["gui"],
            m => unreachable!("Expected screenshot mode full/selection got {m}"),
        };
        Command::new("flameshot")
            .args(flameshot_args.as_slice())
            .status()
            .unwrap();
    }
    eprintln!("No mode provided");
}
