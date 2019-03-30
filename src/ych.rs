// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project
// root for full license information.
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
    pub fn print_json(ych: YCH) -> Result<()> {
        let ych = YCH { id: ych.id, customer: ych.customer,
            order: ych.order, slot: ych.slot, username: ych.username,
            payment: ych.payment
        };

        let json = serde_json::to_string_pretty(&ych)?;

        println!("{}", json);

        Ok(())
    }
}