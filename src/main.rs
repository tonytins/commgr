// Copyright (c) Anthony Leland and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use artm::models::{Opts, Order, Orders};
use chrono::{Local, Datelike};
use clap::Clap;
use std::{fs, fs::{OpenOptions, File}};
use std::io::Write;
use std::path::Path;

fn simple_date() -> String {
    let dt_local = Local::now();
    format!("{}/{}/{}", dt_local.month(), dt_local.day(), dt_local.year())
}

fn csv_manager<S: Into<String>>(file: S, order: &Order) -> File {
    // Create a new time card, if it doesn't exist
    let file_name = &file.into();

    if !Path::new(file_name).exists() {
        File::create(file_name).expect("Error creating file");
    }

    // Append status to time card file
    let mut manger = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .expect("Error writing to file.");

    let contents = fs::read_to_string(file_name)
        .expect("Error opening file");

    if contents.is_empty() {
        let header = if order.ych.is_none() {
            format!("Date,Client,Fee,Payment,Description")
        } else {
            format!("Date,Client,Reference,Fee,Payment,YCH,Slot")
        };

        if let Err(err) = writeln!(manger, "{}", header) {
            eprintln!("Couldn't write to file: {}", err);
        }
    }

    manger
}

fn order_csv(order: Order) {

    let file_name = if order.ych.is_none() {
        format!("commission.csv")
    } else {
        format!("ych.csv")
    };

    // Append status to time card file
    let mut csv = csv_manager(&file_name, &order);
    let record = if order.ych.is_none() || order.slot.is_none() {

        format!("{},{},{},{},\"{}\"", simple_date(), order.client, order.fee,
                order.payment, order.description.unwrap())

    } else {
        format!("{},{},{},{},{},{},{}", simple_date(), order.client, order.reference.unwrap(),
                order.fee, order.payment, order.ych.unwrap(), order.slot.unwrap())
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
