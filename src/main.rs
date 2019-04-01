// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
pub mod ast;

use clap::{App, SubCommand, Arg};
use std::process;
use uuid::Uuid;
use ast::{YCH, Commission};
use chrono::prelude::*;
use crate::ast::Request;

fn main() {
    let error_message = "Application error";
    let exit_code = 1;

    // Todo: figure out test while still keeping yaml feature enabled
    // let yaml = load_yaml!("cli.yml");
    // let matches = App::new(yaml).get_matches();

    let matches = App::new("Art Manager")
        .version("0.1")
        .about("Request, commission, and YCH manager command line interface.")
        .subcommand(SubCommand::with_name("ych")
            .arg(Arg::with_name("buyer")
                .required(true)
                .takes_value(true)
                .long("buyer")
                .short("b"))
            .arg(Arg::with_name("username")
                .required(true)
                .takes_value(true)
                .long("username")
                .short("u"))
            .arg(Arg::with_name("order")
                .required(true)
                .takes_value(true)
                .long("order")
                .short("o"))
            .arg(Arg::with_name("cost")
                .required(true)
                .takes_value(true)
                .long("cost")
                .short("c"))
            .arg(Arg::with_name("slot")
                .takes_value(true)
                .long("slot")
                .short("s"))
            .arg(Arg::with_name("payment")
                .required(true)
                .takes_value(true)
                .long("pay")
                .short("p")))
        .subcommand(SubCommand::with_name("comm")
            .arg(Arg::with_name("buyer")
                .required(true)
                .takes_value(true)
                .long("buyer")
                .short("b"))
            .arg(Arg::with_name("username")
                .required(true)
                .takes_value(true)
                .long("username")
                .short("u"))
            .arg(Arg::with_name("order")
                .required(true)
                .takes_value(true)
                .long("order")
                .short("o"))
            .arg(Arg::with_name("cost")
                .required(true)
                .takes_value(true)
                .long("cost")
                .short("c"))
            .arg(Arg::with_name("payment")
                .required(true)
                .takes_value(true)
                .long("pay")
                .short("p")))
        .get_matches();


    match matches.subcommand() {
        ("ych", Some(ych)) => {
            let ych_buyer = ych.value_of("buyer").unwrap();
            let ych_order = ych.value_of("order").unwrap();
            let ych_slot = ych.value_of("slot").unwrap();
            let ych_cost = ych.value_of("cost").unwrap();
            let ych_contact = ych.value_of("username").unwrap();
            let ych_payment = ych.value_of("payment").unwrap();

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
        },
        ("comm", Some(comm)) => {
            let comm_buyer = comm.value_of("buyer").unwrap();
            let comm_order = comm.value_of("order").unwrap();
            let comm_cost = comm.value_of("cost").unwrap();
            let comm_contact = comm.value_of("username").unwrap();
            let comm_payment = comm.value_of("payment").unwrap();

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
        },
        ("req", Some(req)) => {
            println!("Feature not implemented.")
        },
        _ => unreachable!(),
    }
}
