use std::{fs, path::Path};

const CURRENT: &str = "/.config/wpg/.current";
const LIGHTDM: &str = "/usr/share/lightdm-webkit/themes/modern";
const CACHE: &str = "/.cache/betterlockscreen/current";

fn main() {
    let mut betterlockscreen = std::process::Command::new("betterlockscreen");
    let home = env!("HOME");
    let current = Path::new(&home).join(CURRENT);
    let cache = Path::new(&home).join(CACHE);
    let lightdm = Path::new(LIGHTDM);
    betterlockscreen
        .args(["-u", current.to_str().unwrap(), "--blur", "1"])
        .spawn()
        .expect("Error bluring wallpaper!");
    fs::copy(cache.join("wall_blur.png"), lightdm.join("blur.png"))
        .expect("error copying blurred wallpaper to lightdm");
    fs::copy(current, lightdm.join("wall.png")).expect("error copying wallpaper to lightdm");
}
