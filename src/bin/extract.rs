use std::{
    path::PathBuf,
    process::Command,
};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    archive: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let archive = args.archive;
    let binding = archive.to_str().unwrap()
        .split(".")
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .take(2)
        .collect::<Vec<&str>>();
    let extensions = binding.as_slice();

    let (cmd, mut params) = match (extensions[1], extensions[0]) {
        ("tar", "bz2") | (_, "tbz2") => ("tar", vec!["xvjf"]),
        ("tar", "xz") => ("tar", vec!["xvJf"]),
        ("tar", "gz") | (_,"tgz") => ("tar", vec!["xvzf"]),
        (_, "lzma") => ("unlzma", vec![]),
        (_, "bz2") => ("bunzip2", vec![]),
        (_, "rar") => ("unrar", vec!["x", "-ad"]),
        (_, "gz") => ("gunzip", vec![]),
        (_, "tar") => ("tar", vec!["xvf"]),
        (_, "zip") => ("unzip", vec![]),
        (_, "Z") => ("uncompress", vec![]),
        (_, "7z") => ("7z", vec!["x"]),
        (_, "xz") => ("unxz", vec![]),
        (_, "exe") => ("cabextract", vec![]),
        _ => unreachable!("Unsupported archive format"),
    };

    params.push(&archive.to_str().unwrap());
    Command::new(cmd).args(params).status()?;

    Ok(())
}


