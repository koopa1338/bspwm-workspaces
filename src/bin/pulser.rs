use clap::{Command, FromArgMatches as _, Parser, Subcommand as _};

#[derive(Debug, Parser)]
enum SubCommands {
    VolumeInc { amount: u8 },
    VolumeDec { amount: u8 },
    Mute,
    ToggleSink,
}

#[derive(Debug)]
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

    pub fn toggle_sink(&mut self) {}

    pub fn toggle_mute(&mut self) {}

    pub fn increase_volume(&mut self, amount: u8) {}

    pub fn decrease_volume(&mut self, amount: u8) {}

    fn get_available_sinks() {}

    fn get_volume_of_sink(_sink: usize) {}
}

fn main() -> std::io::Result<()> {
    let cli = Command::new("Built CLI");
    let cli = SubCommands::augment_subcommands(cli);

    let matches = cli.get_matches();
    let derived_subcommands = SubCommands::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap();
    Ok(())
}
