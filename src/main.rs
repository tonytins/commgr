// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
pub mod models;

use clap::{App, crate_version, crate_authors, crate_description, load_yaml};
use std::process;
use uuid::Uuid;
use models::{YCH, Commission, Request};
use chrono::prelude::*;

// Argument names
// =======================
const COMM_ARG: &str = "commission";
const REQ_ARG: &str = "request";
const YCH_ARG: &str = "ych";

const BUYER_ARG: &str = "buyer";
const UNAME_ARG: &str = "username";
const ORDER_ARG: &str = "order";
const PAYMENT_ARG: &str = "payment";
const COST_ARG: &str = "cost";
const SLOT_ARG: &str = "slot";
// const RAFFLE_ARG: &str = "raffle";
// =======================

const ERROR_MSG: &str = "Application error";

fn main() {
    let exit_code = 1;

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    if matches.is_present(YCH_ARG)
    {
        let ych_buyer = matches.value_of(BUYER_ARG).unwrap();
        let ych_order = matches.value_of(ORDER_ARG).unwrap();
        let ych_slot = matches.value_of(SLOT_ARG).unwrap();
        let ych_cost = matches.value_of(COST_ARG).unwrap();
        let ych_contact = matches.value_of(UNAME_ARG).unwrap();
        let ych_payment = matches.value_of(PAYMENT_ARG).unwrap();

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
            println!("{}: {}", ERROR_MSG, err);
            process::exit(exit_code);
        }
    }

    if matches.is_present(COMM_ARG)
    {
        let comm_buyer = matches.value_of(BUYER_ARG).unwrap();
        let comm_order = matches.value_of(ORDER_ARG).unwrap();
        let comm_cost = matches.value_of(COST_ARG).unwrap();
        let comm_contact = matches.value_of(UNAME_ARG).unwrap();
        let comm_payment = matches.value_of(PAYMENT_ARG).unwrap();

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
            println!("{}: {}", ERROR_MSG, err);
            process::exit(exit_code);
        }
    }

    if matches.is_present(REQ_ARG)
    {
        unimplemented!();
        process::exit(exit_code);
    }
}
