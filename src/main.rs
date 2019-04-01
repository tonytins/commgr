// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
pub mod ast;

use clap::{App, SubCommand, Arg};
use std::process;
use uuid::Uuid;
use ast::{YCH, Commission, Request};
use chrono::prelude::*;

// Arguments
// =======================
const YCH_ARG: &str = "ych";
const COMM_ARG: &str = "comm";
const REQ_ARG: &str = "req";

const NAME_ARG: &str = "name";
const UNAME_ARG: &str = "username";
// =======================

fn main() {
    let error_message = "Application error";
    let exit_code = 1;

    // Todo: figure out test while still keeping yaml feature enabled
    // let yaml = load_yaml!("cli.yml");
    // let matches = App::new(yaml).get_matches();

    let matches = App::new("Art Manager")
        .version("0.2")
        .about("Request, commission, and YCH manager command line interface.")
        .arg(Arg::with_name(YCH_ARG))
        .arg(Arg::with_name(COMM_ARG))
        .arg(Arg::with_name(REQ_ARG))
        .arg(Arg::with_name(NAME_ARG)
            .takes_value(true)
            .long(NAME_ARG)
            .short("n"))
        .arg(Arg::with_name(UNAME_ARG)
            .required(true)
            .takes_value(true)
            .long("user")
            .short("u"))
        .arg(Arg::with_name("order")
            .takes_value(true)
            .long("ord")
            .short("o"))
        .arg(Arg::with_name("cost")
            .takes_value(true)
            .long("cost")
            .short("c"))
        .arg(Arg::with_name("slot")
            .takes_value(true)
            .long("slot")
            .short("s")
            .requires("ych"))
        .arg(Arg::with_name("raffle")
            .takes_value(true)
            .long("raf")
            .short("r")
            .requires(YCH_ARG))
        .arg(Arg::with_name("payment")
            .required(true)
            .takes_value(true)
            .long("pay")
            .short("p"))
        .get_matches();

    if matches.value_of(YCH_ARG) {
        let ych_buyer = matches.value_of("buyer").unwrap();
        let ych_order = matches.value_of("order").unwrap();
        let ych_slot = matches.value_of("slot").unwrap();
        let ych_cost = matches.value_of("cost").unwrap();
        let ych_contact = matches.value_of(UNAME_ARG).unwrap();
        let ych_payment = matches.value_of("payment").unwrap();

        if let Err(err) = YCH::write_json(YCH {
            id: Uuid::new_v4()
                .to_hyphenated()
                .to_string(),
            date: Local::now(),
            order: ych_order.to_owned(),
            buyer: ych_buyer.to_owned(),
            cost: ych_cost.to_owned(),
            slot: ych_slot.to_owned(),
            username: ych_contact.to_owned(),
            payment: ych_payment.to_owned(),
        }) {
            println!("{}: {}", error_message, err);
            process::exit(exit_code);
        }
    }

    if matches.value_of("comm")
    {
        let comm_buyer = matches.value_of("buyer").unwrap();
        let comm_order = matches.value_of("order").unwrap();
        let comm_cost = matches.value_of("cost").unwrap();
        let comm_contact = matches.value_of(UNAME_ARG).unwrap();
        let comm_payment = matches.value_of("payment").unwrap();

        if let Err(err) = Commission::write_json(Commission {
            id: Uuid::new_v4()
                .to_hyphenated()
                .to_string(),
            date: Local::now(),
            order: comm_order.to_owned(),
            buyer: comm_buyer.to_owned(),
            cost: comm_cost.to_owned(),
            username: comm_contact.to_owned(),
            payment: comm_payment.to_owned(),
        }) {
            println!("{}: {}", error_message, err);
            process::exit(exit_code);
        }
    }
}
