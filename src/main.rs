// Copyright (c) Anthony Leland and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use cra::{config::get_config,
          utils::{docs_dir, order_manager, personal_manger},
          options::{Opts, Order, Personal, SubCommands}};
use chrono::{Local, Datelike};
use clap::Clap;
use std::io::Write;
use rusty_money::{money, Money};

fn simple_date() -> String {
    let dt_local = Local::now();
    format!("{}/{}/{}", dt_local.month(), dt_local.day(), dt_local.year())
}

fn order_csv(order: Order) {

    let file_name = if order.ych.is_some() {
        format!("ych.csv")
    } else {
        format!("commission.csv")
    };

    // Get configuration information
    let cfg_file = docs_dir("config.toml", false);
    let cfg = get_config(cfg_file);

    // Initialize or manage CSV file
    let mut csv = order_manager(&file_name, &order);

    // Check if we should be using the buyer's or configuration's currency.
    // USD is used as the default currency, if no config.toml is found.
    let order_fee = if order.currency.is_some() {
        money!(order.fee, &order.currency.unwrap().as_str())
    } else {
        money!(order.fee, &cfg.currency.as_str())
    };

    // Check if slots and ych options has something in it
    let record = if order.ych.is_some() && order.slot.is_some() && order.description.is_none() {
        format!("{},\"{}\",{},\"{}\",\"{}\",\"{}\",{}", simple_date(), order.buyer, order.reference.unwrap(),
                order_fee, order.payment, order.ych.unwrap(), order.slot.unwrap())
    } else if order.description.is_some() && order.ych.is_none() && order.slot.is_none() {
        format!("{},\"{}\",\"{}\",\"{}\",\"{}\"", simple_date(), order.buyer, order_fee,
                order.payment, order.description.unwrap())
    } else {
        panic!("Could not determine order.")
    };

    // Finally, append contents to file
    if let Err(err) = writeln!(csv, "{}", record) {
        eprintln!("Couldn't write to file: {}", err);
    }
}

fn personal_csv(pers: Personal) {
    // Initialize or manage CSV file
    let mut csv = personal_manger("personal.csv");

    let record = if pers.reference.is_some() {
        format!("{},{},\"{}\",\"{}\",", simple_date(),pers.name,
                pers.description, pers.reference.unwrap())
    } else {
        format!("{},\"{}\",\"{}\",",simple_date(), pers.name, pers.description)
    };

    // Finally, append contents to file
    if let Err(err) = writeln!(csv, "{}", record) {
        eprintln!("Couldn't write to file: {}", err);
    }
}

fn main() {
    let matches: Opts = Opts::parse();

    match matches.subcmds {
        SubCommands::Order(order) => {
            order_csv(order);
        }
        SubCommands::Pers(personal) => {
            personal_csv(personal);
        }
    }
}
