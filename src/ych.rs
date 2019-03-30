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
    pub contact: String,
    /// Payment information (paypal, crypto, ect)
    pub payment: String,
}

impl YCH {
    pub fn print_json(ych: YCH) -> Result<()> {
        let ych = YCH { id: ych.id, customer: ych.customer,
            order: ych.order, slot: ych.slot, contact: ych.contact,
            payment: ych.payment
        };

        let json = serde_json::to_string_pretty(&ych)?;

        println!("{}", json);

        Ok(())
    }
}