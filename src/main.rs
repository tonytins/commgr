// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
pub mod models;
pub mod cmds;

use chrono::prelude::*;
use clap::{crate_authors, crate_description, crate_version, load_yaml, App};
use models::{Commission, Request, YCH};
use std::process;
use cmds::*;
use uuid::Uuid;

fn main() {
    let exit_code = 1;
    let yaml = load_yaml!("artm.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    if let Some(ych) = matches.subcommand_matches(YCH_CMD) {
        let client = ych.value_of(CLIENT_OPT).unwrap();
        let art = ych.value_of(ART_OPT).unwrap();
        let reference = ych.value_of(REF_OPT).unwrap();
        let slot = ych.value_of(SLOT_OPT).unwrap();
        let price = ych.value_of(PRICE_OPT).unwrap();
        let contact = ych.value_of(CONTACT_OPT).unwrap();
        let payment = ych.value_of(PAYMENT_OPT).unwrap();

        if ych.is_present(DEBUG_FLAG) {
            if let Err(err) = YCH::print_ych(YCH {
                id: Uuid::new_v4().to_string(),
                date: Local::now(),
                art: art.to_owned(),
                customer: client.to_owned(),
                reference: reference.to_owned(),
                price: price.to_owned(),
                slot: slot.to_owned(),
                contact: contact.to_owned(),
                payment: payment.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        } else {
            if let Err(err) = YCH::write_ych(YCH {
                id: Uuid::new_v4().to_string(),
                date: Local::now(),
                art: art.to_owned(),
                reference: reference.to_owned(),
                customer: client.to_owned(),
                price: price.to_owned(),
                slot: slot.to_owned(),
                contact: contact.to_owned(),
                payment: payment.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }

        if let Some(_raf) = matches.subcommand_matches(RAFFLE_CMD) {
            unimplemented!();
        }
    }

    if matches.is_present(COMM_FLAG) {
        let client = matches.value_of(CLIENT_OPT).unwrap();
        let art = matches.value_of(ART_OPT).unwrap();
        let cost = matches.value_of(PRICE_OPT).unwrap();
        let contact = matches.value_of(CONTACT_OPT).unwrap();
        let payment = matches.value_of(PAYMENT_OPT).unwrap();
        let description = matches.value_of(DESC_OPT).unwrap();

        if matches.is_present(DEBUG_FLAG) {
            if let Err(err) = Commission::print_comm(Commission {
                id: Uuid::new_v4().to_string(),
                date: Local::now(),
                customer: client.to_owned(),
                art: art.to_owned(),
                price: cost.to_owned(),
                contact: contact.to_owned(),
                payment: payment.to_owned(),
                description: description.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        } else {
            if let Err(err) = Commission::write_comm(Commission {
                id: Uuid::new_v4().to_string(),
                date: Local::now(),
                customer: client.to_owned(),
                art: art.to_owned(),
                price: cost.to_owned(),
                contact: contact.to_owned(),
                payment: payment.to_owned(),
                description: description.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }
    }

    if matches.is_present(REQ_FLAG) {
        let client = matches.value_of(CLIENT_OPT).unwrap();
        let art = matches.value_of(ART_OPT).unwrap();
        let contact = matches.value_of(CONTACT_OPT).unwrap();
        let description = matches.value_of(DESC_OPT).unwrap();

        if matches.is_present(DEBUG_FLAG) {
            if let Err(err) = Request::print_req(Request {
                id: Uuid::new_v4().to_string(),
                date: Local::now(),
                art: art.to_owned(),
                customer: client.to_owned(),
                contact: contact.to_owned(),
                description: description.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        } else {
            if let Err(err) = Request::write_req(Request {
                id: Uuid::new_v4().to_string(),
                date: Local::now(),
                art: art.to_owned(),
                customer: client.to_owned(),
                contact: contact.to_owned(),
                description: description.to_owned(),
            }) {
                println!("{}: {}", ERROR_MSG, err);
                process::exit(exit_code);
            }
        }
    }
}
