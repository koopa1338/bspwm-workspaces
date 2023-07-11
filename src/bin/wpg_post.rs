use std::{fs, io::ErrorKind, path::Path};

const CURRENT: &str = ".config/wpg/.current";
const LIGHTDM: &str = "/usr/share/lightdm-webkit/themes/modern";
const CACHE: &str = ".cache/betterlockscreen/current";

fn main() {
    let home = env!("HOME");
    let current = Path::new(&home).join(CURRENT);
    let lightdm = Path::new(LIGHTDM);
    if lightdm.exists() {
        let mut betterlockscreen = std::process::Command::new("betterlockscreen");
        match betterlockscreen
            .args(["-u", current.to_str().unwrap(), "--blur", "1"])
            .spawn()
        {
            Ok(_) => {
                let cache = Path::new(&home).join(CACHE);
                std::thread::spawn(move || {
                    fs::copy(cache.join("wall_blur.png"), lightdm.join("blur.png"))
                        .expect("error copying blurred wallpaper to lightdm");
                })
                .join()
                .expect("Thread error: copying blurred wallpaper to lightdm");
                std::thread::spawn(|| {
                    fs::copy(current, lightdm.join("wall.png"))
                        .expect("error copying wallpaper to lightdm");
                })
                .join()
                .expect("Thread error: copying wallpaper to lightdm");
            }
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    eprintln!("`betterlockscreen not installed`")
                } else {
                    eprintln!("betterlockscreen failed");
                }
            }
        }
    } else {
        eprintln!("lightdm theme path {lightdm:?} does not exist");
    }
}
