use clap::{command, Arg};

use rip8::run;

fn main() {
    let app = command!().about("A CHIP-8 Emulator written in rust.").arg(
        Arg::new("rom")
            .required(true)
            .short('r')
            .long("rom")
            .long_help("the path to the ROM file")
            .takes_value(true),
    );

    let matches = app.get_matches();
    run(&matches.get_one::<String>("rom").unwrap());
}
