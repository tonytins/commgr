// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use chrono::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::OpenOptions;
use std::io::Write;
use core::fmt::Debug;

const ARTM_EXT: &str = "artm";

#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    YCH,
    Commission,
    Request,
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    pub name: String,
    pub contact: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Payment information (paypal, crypto, ect)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Art {
    pub id: String,
    /// Use local time
    pub date: DateTime<Local>,
    pub version: String,
    pub name: String,
    pub category: Option<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

impl Default for Category {
    fn default() -> Self { Category::Unknown }
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            name: "".to_string(),
            contact: "".to_string(),
            reference: None,
            payment: None
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S)
        -> Customer where S: Into<String> {
        self.name = name.into();
        self
    }

    pub fn contact<S: Into<String>>(mut self, cont: S)
        -> Customer where S: Into<String> {
        self.contact = cont.into();
        self
    }

    pub fn reference<S: Into<String>>(mut self, reference: S)
                                    -> Customer where S: Into<String> {
        self.reference = Some(reference.into());
        self
    }

    pub fn payment<S: Into<String>>(mut self, pay: S)
        -> Customer where S: Into<String> {
        self.payment = Some(pay.into());
        self
    }
}

impl Art {

    pub fn new() -> Art {
        Art {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            date: Local::now(),
            version: "0.1".to_string(),
            name: "".to_string(),
            category: Some(Category::default()),
            customer: None,
            ticket: None,
            slot: None,
            price: None,
            description: None
        }
    }

    pub fn secure_id(mut self) -> Art {
        let hash = Uuid::new_v5(&Uuid::NAMESPACE_OID,format!("{}+artm", self.name).as_bytes())
            .to_hyphenated().to_string();

        self.id = hash;

        self
    }

    pub fn customer(mut self, cust: Customer) -> Art {
        self.customer = Some(cust.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S)
        -> Art where S: Into<String> {
        self.name = name.into();
        self
    }

    pub fn price<S: Into<String>>(mut self, price: S)
        -> Art where S: Into<String> {
        self.price = Some(price.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, desc: S)
        -> Art where S: Into<String> {
        self.description = Some(desc.into());
        self
    }

    pub fn slot<S: Into<String>>(mut self, slot: S)
                                        -> Art where S: Into<String> {
        self.slot = Some(slot.into());
        self
    }

    pub fn reference<S: Into<String>>(mut self, desc: S)
                                        -> Art where S: Into<String> {
        self.description = Some(desc.into());
        self
    }

    pub fn category(mut self, cat: &str) -> Art {
        match cat {
            "ych" => self.category = Some(Category::YCH),
            "comm" => self.category = Some(Category::Commission),
            "req" => self.category = Some(Category::Request),
            _ => self.category = Some(Category::default()),
        }
        self
    }

    pub fn write_file(&self, debug: bool) -> Result<()> {
        let json_string = serde_json::to_string_pretty(self)?;

        match debug {
            true => println!("{}", json_string),
            false => {
                let cat = &self.category;
                let slot = self.slot.to_owned().unwrap();
                let name = self.name.to_owned();
                let mut file_name = String::new();

                match cat {
                    Some(Category::YCH) => {
                        file_name = format!("{} - slot {}.{}",
                                                name, slot, ARTM_EXT);
                    }
                    _ => {
                        file_name = format!("{}.{}",
                                            name, ARTM_EXT);
                    }
                }

                let mut file = OpenOptions::new().write(true)
                    .create_new(true)
                    .open(file_name)
                    .expect("Could not open file.");

                if let Err(err) = writeln!(file, "{}", format!("{}", json_string)) {
                    eprintln!("Could not write to file. {}", err);
                }
            }
        }

        Ok(())
    }
}