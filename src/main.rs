// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
pub mod models;
pub mod tstr;

use clap::{App, crate_version, crate_authors,
           crate_description, load_yaml};
use std::process;
use uuid::Uuid;
use models::{YCH, Commission, Request};
use chrono::prelude::*;
use tstr::*;

fn main() {
    let exit_code = 1;
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    if matches.is_present(YCH_FLAG)
    {
        let client = matches.value_of(CLIENT_OPT).unwrap();
        let art = matches.value_of(ART_OPT).unwrap();
        let reference = matches.value_of(REF_OPT).unwrap();
        let slot = matches.value_of(SLOT_OPT).unwrap();
        let price = matches.value_of(PRICE_OPT).unwrap();
        let contact = matches.value_of(UNAME_OPT).unwrap();
        let payment = matches.value_of(PAYMENT_OPT).unwrap();

        if matches.is_present(DEBUG_FLAG)
        {
            if let Err(err) = YCH::print_ych(YCH {
                id: Uuid::new_v4()
                    .to_hyphenated()
                    .to_string(),
                date: Local::now(),
                art: art.to_owned(),
                client: client.to_owned(),
                reference: reference.to_owned(),
                cost: price.to_owned(),
                slot: slot.to_owned(),
                username: contact.to_owned(),
                payment: payment.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }
        else
        {
            if let Err(err) = YCH::write_ych(YCH {
                id: Uuid::new_v4()
                    .to_hyphenated()
                    .to_string(),
                date: Local::now(),
                art: art.to_owned(),
                reference: reference.to_owned(),
                client: client.to_owned(),
                cost: price.to_owned(),
                slot: slot.to_owned(),
                username: contact.to_owned(),
                payment: payment.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }
    }

    if matches.is_present(COMM_FLAG)
    {
        let client = matches.value_of(CLIENT_OPT).unwrap();
        let art = matches.value_of(ART_OPT).unwrap();
        let cost = matches.value_of(PRICE_OPT).unwrap();
        let contact = matches.value_of(UNAME_OPT).unwrap();
        let payment = matches.value_of(PAYMENT_OPT).unwrap();
        let description = matches.value_of(DESC_OPT).unwrap();

        if matches.is_present(DEBUG_FLAG)
        {
            if let Err(err) = Commission::print_comm(Commission {
                id: Uuid::new_v4()
                    .to_hyphenated()
                    .to_string(),
                date: Local::now(),
                client: client.to_owned(),
                art: art.to_owned(),
                cost: cost.to_owned(),
                username: contact.to_owned(),
                payment: payment.to_owned(),
                description: description.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }
        else
        {
            if let Err(err) = Commission::write_comm(Commission {
                id: Uuid::new_v4()
                    .to_hyphenated()
                    .to_string(),
                date: Local::now(),
                client: client.to_owned(),
                art: art.to_owned(),
                cost: cost.to_owned(),
                username: contact.to_owned(),
                payment: payment.to_owned(),
                description: description.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }
    }

    if matches.is_present(REQ_FLAG)
    {
        let client = matches.value_of(CLIENT_OPT).unwrap();
        let art = matches.value_of(ART_OPT).unwrap();
        let contact = matches.value_of(UNAME_OPT).unwrap();
        let description = matches.value_of(DESC_OPT).unwrap();

        if matches.is_present(DEBUG_FLAG)
        {
            if let Err(err) = Request::print_req(Request {
                id: Uuid::new_v4()
                    .to_hyphenated()
                    .to_string(),
                date: Local::now(),
                art: art.to_owned(),
                client: client.to_owned(),
                username: contact.to_owned(),
                description: description.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }
        else
        {
            if let Err(err) = Request::write_req(Request {
                id: Uuid::new_v4()
                    .to_hyphenated()
                    .to_string(),
                date: Local::now(),
                art: art.to_owned(),
                client: client.to_owned(),
                username: contact.to_owned(),
                description: description.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }
    }

    if let Some(_) = matches.subcommand_matches(RAFFLE_CMD)
    {
        unimplemented!();
    }
}
