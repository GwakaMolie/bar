use systemstat::{Platform, System};
use unixbar::*;

fn main() {

    let mut bar = UnixBar::new(AwesomeFormatter::new());

    bar
    // Volume
    .add(Volume::new(default_volume(), |volume| {
        if volume.muted {
            bfmt![text["mute"]]
        } else {
            let volume = (volume.volume * 100.).round() as i32;
            bfmt![fmt["V {}%", volume]]
        }
    }))

    // Battery
    .add(Periodic::new(Duration::from_secs(30), || {
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
    ))

    // MPD
    // .register_fn("prev", move || { MPDMusic::new().prev(); })
    // .register_fn("play_pause", move || { MPDMusic::new().play_pause(); })
    // .register_fn("next", move || { MPDMusic::new().next(); })
    // .add(Music::new(MPDMusic::new(),
    //     |song| bfmt![ fmt["{}", song.artist]]
    // ))

    // CPU Usage
    .add(Delayed::new(Duration::from_secs(5),
        || System::new().cpu_load_aggregate().unwrap(),
        |res| match res {
            Ok(cpu) => bfmt![fmt[" C {}% ", ((1.0 - cpu.idle) * 100.0) as i32]],
            Err(_) => bfmt![fg["#bb1155"] text["error"]],
    }));

    bar.run_no_stdin();
}
