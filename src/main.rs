use clap::{command, Arg};

use log::debug;
use rip8::run;

fn main() {
    env_logger::init();
    debug!("RIP");

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
