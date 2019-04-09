// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
pub mod art;
pub mod cli;

use art::{comm, req, ych};
use clap::{crate_authors, crate_description, crate_version, load_yaml, App};
use simplelog::*;
use std::fs::File;
use std::env;
use cli::*;
use dotenv::{dotenv};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let db_file = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");

    SqliteConnection::establish(&db_file)
        .expect(&format!("Error connecting to {},", db_file))
}

fn main() {
    let log_file = "artm.log";

    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create(log_file).unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file).unwrap(),
        ),
    ]).unwrap();

    let matches = cli_arg();

    let debug = matches.is_present(DEBUG_FLAG);
    let name = matches.value_of(NAME_OPT).unwrap();

    let mut cust_name = "";
    let mut desc = "";
    let mut contact = "";
    let mut slot = "";
    let mut payment = "";
    let mut price = "";
    let mut reference = "";

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

                        ych(
                            name, price, slot, reference, cust_name, payment, contact, debug,
                        );
                    }
                    false => {
                        desc = matches.value_of(DESC_OPT).unwrap();

                        comm(name, price, desc, cust_name, payment, contact, debug);
                    }
                }
            }
            false => {
                desc = matches.value_of(DESC_OPT).unwrap();

                req(name, desc, cust_name, contact, debug);
            }
        }
    } else {
        /*
        let slots = matches.value_of(SLOTS_OPT).unwrap();
        let tickets = matches.value_of(TICKETS_OPT).unwrap();

        raffle(name, tickets.parse().unwrap(), slots.parse().unwrap(), debug);
        */
        unimplemented!();
    }

}
