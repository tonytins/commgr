// Copyright (c) Anthony Leland and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use artm::{config::get_config,
           utils::{docs_dir, csv_manager},
           options::{Opts, Order, Orders}};
use chrono::{Local, Datelike};
use clap::Clap;
use std::{fs, fs::{OpenOptions, File}};
use std::io::Write;
use std::path::Path;
use directories::UserDirs;
use rusty_money::{money, Money, Currency};

fn simple_date() -> String {
    let dt_local = Local::now();
    format!("{}/{}/{}", dt_local.month(), dt_local.day(), dt_local.year())
}

fn order_csv(order: Order) {

    let file_name = if order.ych.is_none() {
        format!("commission.csv")
    } else {
        format!("ych.csv")
    };

    // Append status to order file
    let cfg_file = docs_dir("config.toml", false);
    let cfg = get_config(cfg_file);
    let mut csv = csv_manager(&file_name, &order);
    let record = if order.ych.is_none() || order.slot.is_none() {

        format!("{},{},{},{},\"{}\"", simple_date(), order.client, money!(order.fee, &cfg.currency),
                order.payment, order.description.unwrap())

    } else {
        format!("{},{},{},{},{},{},{}", simple_date(), order.client, order.reference.unwrap(),
                money!(order.fee, &cfg.currency), order.payment, order.ych.unwrap(), order.slot.unwrap())
    };

    if let Err(err) = writeln!(csv, "{}", record) {
        eprintln!("Couldn't write to file: {}", err);
    }
}

fn main() {
    let matches: Opts = Opts::parse();

    match matches.order {
        Orders::Order(order) => {
            order_csv(order);
        }
    }
}
