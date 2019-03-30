pub mod ych;

use clap::{App, SubCommand, Arg};
use std::process;
use uuid::Uuid;
use ych::YCH;

fn main() {
    let matches = App::new("Codename: Purple Fen")
        .version("0.1")
        .about("Request, commission, and YCH manager command line interface.")
        .subcommand(SubCommand::with_name("ych")
            .arg(Arg::with_name("name")
                .takes_value(true)
                .long("name")
                .short("n"))
            .arg(Arg::with_name("slot")
                .takes_value(true)
                .long("slot")
                .short("s")))
        .get_matches();

    match matches.subcommand() {
        ("ych", Some(ych)) => {
            let ych_name = ych.value_of("name").unwrap();
            let ych_slot = ych.value_of("slot").unwrap();

            if let Err(e) = YCH::print_json(YCH {
                id: Uuid::new_v4()
                    .to_hyphenated()
                    .to_string(),
                name: ych_name.to_owned(),
                slot: ych_slot.to_owned()
            }) {
                println!("Application error: {}", e);
                process::exit(1);
            }
        },
        _ => unreachable!(),
    }
}
