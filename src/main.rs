use std::env;

use systemstat::{Platform, System};
use unixbar::*;

#[derive(Default)]
struct Config {
    volume: bool,
    battery: bool,
    cpu_load: bool,
    mpd: bool,
}

impl Config {
    fn from_args() -> Self {
        let mut config = Config::default();

        for arg in env::args().skip(1) {
            match arg.as_str() {
                "-v" => config.volume = true,
                "-b" => config.battery = true,
                "-c" => config.cpu_load = true,
                "-m" => config.mpd = true,
                _ => (),
            }
        }
        config
    }
}

fn main() {
    let mut bar = UnixBar::new(AwesomeFormatter::new());
    let config = Config::from_args();

    // MPD
    if config.mpd {
        bar.register_fn("prev", move || {
            MPDMusic::new().prev();
        })
        .register_fn("play_pause", move || {
            MPDMusic::new().play_pause();
        })
        .register_fn("next", move || {
            MPDMusic::new().next();
        })
        .add(Music::new(MPDMusic::new(), |song| {
            // bfmt![ fmt["{} ", song.title ]]
            if let Some(playing) = song.playback.map(|playback| playback.playing) {
                if playing {
                    bfmt![fg["#bbbbbb"] fmt[" {} ", song.title]]
                } else {
                    bfmt![fg["#666666"] fmt[" {} ", song.title]]
                }
            } else {
                bfmt![fg["#bbbbbb"] text["[start music]"]]
            }
        }));
    }
    if config.volume {
        // Volume
        bar.add(Volume::new(default_volume(), |volume| {
            if volume.muted {
                bfmt![text["mute"]]
            } else {
                let volume = (volume.volume * 100.).round() as i32;
                bfmt![fmt["V {}%", volume]]
            }
        }));
    }

    // Battery
    if config.battery {
        bar.add(Periodic::new(Duration::from_secs(30), || {
        let system = System::new();
        let symbol = system.on_ac_power().map(|on_ac_power| if on_ac_power { "" } else { "B" });
        match (symbol, system.battery_life()) {
            (Ok(symbol), Ok(battery)) =>
                bfmt![ pad[1] fg["#ff5555"] fmt["{} {}%", symbol, (battery.remaining_capacity * 100.).round() as i32]],
            (Ok(symbol), Err(_)) =>
                bfmt![ pad[1] text[symbol]],
            (Err(err), _) =>
                bfmt![fg["#bb1155"] pad[1] text[err.to_string()]],
        }}
    ));
    }

    // CPU Usage
    if config.cpu_load {
        bar.add(Delayed::new(
            Duration::from_secs(5),
            || System::new().cpu_load_aggregate().unwrap(),
            |res| match res {
                Ok(cpu) => bfmt![fmt[" C {}% ", ((1.0 - cpu.idle) * 100.0) as i32]],
                Err(_) => bfmt![fg["#bb1155"] text["error"]],
            },
        ));
    }

    bar.run_no_stdin();
}
