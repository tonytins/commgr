// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project
// root for full license information.
use std::io::Write;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct YCH {
    pub id: String,
    pub customer: String,
    /// The YCH that was won
    pub order: String,
    /// Slot the customer won in the auction
    pub slot: String,
    pub username: String,
    /// Payment information (paypal, crypto, ect)
    pub payment: String,
}

impl YCH {
    pub fn write_json(ych: YCH) -> Result<()> {
        let ych = YCH { id: ych.id, customer: ych.customer,
            order: ych.order, slot: ych.slot, username: ych.username,
            payment: ych.payment
        };

        let json_string = serde_json::to_string_pretty(&ych);
        let order_string = ych.order.to_string().to_lowercase();
        let slot_string = ych.slot.to_string().to_lowercase();
        let cust_string = ych.customer.to_string().to_lowercase();
        let file_name = format!("{} - {} - {}.json", order_string, slot_string, cust_string);
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