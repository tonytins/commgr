// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
pub mod models;
pub mod cmds;

use chrono::prelude::*;
use clap::{crate_authors, crate_description, crate_version, load_yaml, App};
use models::{Category, Customer, Art};
use std::process;
use cmds::*;

fn main() {
    let exit_code = 1;
    let yaml = load_yaml!("artm.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    let cust = matches.value_of(CLIENT_OPT).unwrap();
    let art = matches.value_of(ART_OPT).unwrap();
    let reference = matches.value_of(REF_OPT).unwrap();
    let slot = matches.value_of(SLOT_OPT).unwrap();
    let price = matches.value_of(PRICE_OPT).unwrap();
    let contact = matches.value_of(CONTACT_OPT).unwrap();
    let payment = matches.value_of(PAYMENT_OPT).unwrap();
    let category = matches.value_of(CAT_OPT).unwrap();

    if let Err(err) = Art::new()
        .price(price)
        .name(art)
        .category(category)
        .slot(slot)
        .customer(Customer::new()
            .name(cust)
            .payment(payment)
            .contact(contact)
            .reference(reference))
        .secure_id()
        .write_file(matches.is_present(DEBUG_FLAG)) {
        println!("{}: {}", ERROR_MSG, err);
        process::exit(exit_code);
    }
}
