use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Commands {
    VolumeInc(u8),
    VolumeDec(u8),
    Mute,
    ToggleSink,
}

struct State {
    available_sinks: Vec<usize>,
    current_sink: usize,
    current_volume: u8,
    is_muted: bool,
}

impl State {
    fn new() -> Self {
        Self {
            available_sinks: vec![],
            current_sink: 0,
            current_volume: 100,
            is_muted: false,
        }
    }

    pub fn toggle_sink(&mut self){}

    pub fn toggle_mute(&mut self){}

    pub fn increase_volume(&mut self, amount: u8){}

    pub fn decrease_volume(&mut self, amount: u8){}
    
    fn get_available_sinks() {}

    fn get_volume_of_sink(_sink: usize) {}

    fn get_mute_state_of_sink(_sink: usize) {}

    fn get_volume_of_sink(_sink: usize) {}
}

fn main() -> std::io::Result<()> {
    let args = Commands::parse();

    Ok(())
}
