// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use std::io::Write;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use chrono::prelude::*;

const AMY_EXT: &str = "amy";
const AMC_EXT: &str = "amc";
const AMR_EXT: &str = "amr";

#[derive(Serialize, Deserialize, Debug)]
pub struct YCH {
    pub id: String,
    /// Uses the local time
    pub date: DateTime<Local>,
    pub buyer: String,
    /// The YCH
    pub order: String,
    /// Slot the customer won in the auction
    pub slot: String,
    pub username: String,
    pub cost: String,
    /// Payment information (paypal, crypto, ect)
    pub payment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commission {
    pub id: String,
    /// Uses the local time
    pub date: DateTime<Local>,
    pub buyer: String,
    /// The commission
    pub order: String,
    pub username: String,
    pub cost: String,
    /// Payment information (paypal, crypto, ect)
    pub payment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub id: String,
    /// Uses the local time
    pub date: DateTime<Local>,
    pub customer: String,
    /// The request
    pub name: String,
    pub username: String,
}


impl YCH {
    pub fn write_json(ych: YCH) -> Result<()> {
        let new_ych = YCH { id: ych.id, date: ych.date, buyer: ych.buyer,
            order: ych.order, slot: ych.slot, username: ych.username,
            payment: ych.payment, cost: ych.cost
        };

        let json_string = serde_json::to_string_pretty(&new_ych);
        let order_string = new_ych.order.to_owned().to_lowercase();
        let slot_string = new_ych.slot.to_owned().to_lowercase();
        let cust_string = new_ych.buyer.to_owned().to_lowercase();
        let file_name = format!("{} - {} - {}.{}",
                                order_string, slot_string, cust_string, AMY_EXT);
        let mut file = OpenOptions::new().write(true)
            .create_new(true)
            .open(file_name)
            .expect("Could not open file.");

        if let Err(err) = writeln!(file, "{}", format!("{}", json_string?)) {
            eprintln!("Couldn't write to file. {}", err);
        }

        Ok(())
    }
}

impl Commission {
    pub fn write_json(comm: Commission) -> Result<()> {
        let new_comm = Commission {
            id: comm.id, date: comm.date, buyer: comm.buyer,
            order: comm.order, username: comm.username, payment: comm.payment,
            cost: comm.cost
        };

        let json_string = serde_json::to_string_pretty(&new_comm);
        let order_string = new_comm.order.to_string().to_lowercase();
        let cust_string = new_comm.buyer.to_string().to_lowercase();
        let file_name = format!("{} - {}.{}", order_string, cust_string, AMC_EXT);
        let mut file = OpenOptions::new().write(true)
            .create_new(true)
            .open(file_name)
            .expect("Could not open file.");

        if let Err(err) = writeln!(file, "{}", format!("{}", json_string?)) {
            eprintln!("Couldn't write to file. {}", err);
        }

        Ok(())
    }
}

impl Request {
    pub fn write_json(req: Request) -> Result<()> {
        let new_comm = Request {
            id: req.id, date: req.date, customer: req.customer,
            username: req.username, name: req.name,
        };

        let json_string = serde_json::to_string_pretty(&new_comm);
        let name_string = new_comm.name.to_string().to_lowercase();
        let cust_string = new_comm.customer.to_string().to_lowercase();
        let file_name = format!("{} - {}.{}", name_string, cust_string, AMR_EXT);
        let mut file = OpenOptions::new().write(true)
            .create_new(true)
            .open(file_name)
            .expect("Could not open file.");

        if let Err(err) = writeln!(file, "{}", format!("{}", json_string?)) {
            eprintln!("Couldn't write to file. {}", err);
        }

        Ok(())
    }
}