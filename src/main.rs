// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project
// root for full license information.
mod art;
mod flags;

use flags::*;
use simplelog::*;
use art::{ych, comm, req};
use self_update::backends::github::Update;
use clap::{crate_authors, crate_description,
           crate_version, load_yaml, App};
use std::{fs::File, error::Error, process::exit};

const CREDITS: &str = "artm is licensed under the BSD 3-clause license.\n\
    ###########################\
    config - MIT/Apache-2.0\n\
    self_update - MIT\n\
    log - MIT/Apache-2.0\n\
    simplelog - MIT/Apache-2.0\n\
    clap - MIT\n\
    UUID - MIT/Apache-2.0\n\
    serde - MIT/Apache-2.0\n\
    chrono - MIT/Apache-2.0\n";

fn main() {
    let log_file = "artm.log";
    let about_desc = format!("{}\n{}", crate_description!(), CREDITS);

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn,
                            Config::default())
                .unwrap(),
            WriteLogger::new(LevelFilter::Info,
                             Config::default(),
                             File::create(log_file)
                                 .unwrap()),

        ]
    ).unwrap();

    let yaml = load_yaml!("artm.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    let debug = matches.is_present(DEBUG_FLAG);
    let name = matches.value_of(NAME_OPT).unwrap();

    let mut cust_name = "";
    let mut desc= "";
    let mut contact = "";
    let mut slot = "";
    let mut payment = "";
    let mut price = "";
    let mut reference = "";
    let mut version = "";

    if matches.is_present(UPDATE_CMD) {

        version = matches.value_of(VERSION_OPT).unwrap();

        match matches.is_present(VERSION_OPT) {
            true => {
                unimplemented!();
            }
            false => {
                unimplemented!();
            }
        }
    }

    if matches.is_present(CREDITS_FLAG) {
        println!("{}", CREDITS);
    }

    if !matches.is_present(RAFFLE_CMD) {
        cust_name = matches.value_of(CUST_NAME_OPT).unwrap();
        contact = matches.value_of(CONTACT_OPT).unwrap();

        match matches.is_present(PAYMENT_OPT) {
            true => {
                payment = matches.value_of(PAYMENT_OPT).unwrap();
                price = matches.value_of(PRICE_OPT).unwrap();

                match matches.is_present(SLOT_OPT) || matches.is_present(REF_OPT) {
                    true => {
                        slot = matches.value_of(SLOT_OPT).unwrap();
                        reference = matches.value_of(REF_OPT).unwrap();

                        ych(name, price, slot, reference, cust_name, payment, contact, debug);
                    }
                    false => {
                        desc = matches.value_of(DESC_OPT).unwrap();

                        comm(name, price, desc, cust_name,payment, contact, debug);
                    }
                }
            }
            false => {
                desc = matches.value_of(DESC_OPT).unwrap();

                req(name, desc, cust_name, contact, debug);
            }
        }
    }
    else {
        /*
        let slots = matches.value_of(SLOTS_OPT).unwrap();
        let tickets = matches.value_of(TICKETS_OPT).unwrap();

        raffle(name, tickets.parse().unwrap(), slots.parse().unwrap(), debug);
        */
        unimplemented!();
    }
}
