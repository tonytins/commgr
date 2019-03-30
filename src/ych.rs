use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct YCH {
    pub id: String,
    pub name: String,
    pub slot: String,
}

impl YCH {
    pub fn print_json(ych: YCH) -> Result<()> {
        let ych = YCH { id: ych.id,
            name: ych.name, slot: ych.slot
        };

        let json = serde_json::to_string(&ych)?;

        println!("{}", json);

        Ok(())
    }
}