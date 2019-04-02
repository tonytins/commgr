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

    if let Some(ych) = matches.subcommand_matches(YCH_FLAG)
    {
        let client = ych.value_of(CLIENT_OPT).unwrap();
        let art = ych.value_of(ART_OPT).unwrap();
        let reference = ych.value_of(REF_OPT).unwrap();
        let slot = ych.value_of(SLOT_OPT).unwrap();
        let price = ych.value_of(PRICE_OPT).unwrap();
        let contact = ych.value_of(CONTACT_OPT).unwrap();
        let payment = ych.value_of(PAYMENT_OPT).unwrap();

        if ych.is_present(DEBUG_FLAG)
        {
            if let Err(err) = YCH::print_ych(YCH {
                id: Uuid::new_v4()
                    .to_string(),
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
        }
        else
        {
            if let Err(err) = YCH::write_ych(YCH {
                id: Uuid::new_v4()
                    .to_string(),
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
    }

    if let Some(comm) = matches.subcommand_matches(COMM_FLAG)
    {
        let client = comm.value_of(CLIENT_OPT).unwrap();
        let art = comm.value_of(ART_OPT).unwrap();
        let cost = comm.value_of(PRICE_OPT).unwrap();
        let contact = comm.value_of(CONTACT_OPT).unwrap();
        let payment = comm.value_of(PAYMENT_OPT).unwrap();
        let description = comm.value_of(DESC_OPT).unwrap();

        if comm.is_present(DEBUG_FLAG)
        {
            if let Err(err) = Commission::print_comm(Commission {
                id: Uuid::new_v4()
                    .to_string(),
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
        else
        {
            if let Err(err) = Commission::write_comm(Commission {
                id: Uuid::new_v4()
                    .to_string(),
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

    if let Some(req) = matches.subcommand_matches(REQ_FLAG)
    {
        let client = req.value_of(CLIENT_OPT).unwrap();
        let art = req.value_of(ART_OPT).unwrap();
        let contact = req.value_of(CONTACT_OPT).unwrap();
        let description = req.value_of(DESC_OPT).unwrap();

        if req.is_present(DEBUG_FLAG)
        {
            if let Err(err) = Request::print_req(Request {
                id: Uuid::new_v4()
                    .to_string(),
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
        else
        {
            if let Err(err) = Request::write_req(Request {
                id: Uuid::new_v4()
                    .to_string(),
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

    if let Some(raf) = matches.subcommand_matches(RAFFLE_CMD)
    {
        unimplemented!();
    }
}
