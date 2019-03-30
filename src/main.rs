pub mod ych;

use clap::{App, SubCommand, Arg};
use std::process;
use uuid::Uuid;
use ych::YCH;

fn main() {
    // Todo: Move to YAML in future
    let matches = App::new("Codename: Purple Fen")
        .version("0.1")
        .about("Request, commission, and YCH manager command line interface.")
        .subcommand(SubCommand::with_name("ych")
            .arg(Arg::with_name("customer")
                .takes_value(true)
                .long("cust")
                .short("c"))
            .arg(Arg::with_name("username")
                .takes_value(true)
                .long("username")
                .short("u"))
            .arg(Arg::with_name("order")
                .takes_value(true)
                .long("order")
                .short("o"))
            .arg(Arg::with_name("slot")
                .takes_value(true)
                .long("slot")
                .short("s"))
            .arg(Arg::with_name("payment")
                .takes_value(true)
                .long("pay")
                .short("p")))
        .get_matches();

    match matches.subcommand() {
        ("ych", Some(ych)) => {
            let ych_cust = ych.value_of("customer").unwrap();
            let ych_order = ych.value_of("order").unwrap();
            let ych_slot = ych.value_of("slot").unwrap();
            let ych_contact = ych.value_of("contact").unwrap();
            let ych_payment = ych.value_of("payment").unwrap();

            if let Err(e) = YCH::print_json(YCH {
                id: Uuid::new_v4()
                    .to_hyphenated()
                    .to_string(),
                order: ych_order.to_owned(),
                customer: ych_cust.to_owned(),
                slot: ych_slot.to_owned(),
                contact: ych_contact.to_owned(),
                payment: ych_payment.to_owned(),
            }) {
                println!("Application error: {}", e);
                process::exit(1);
            }
        },
        _ => unreachable!(),
    }
}
