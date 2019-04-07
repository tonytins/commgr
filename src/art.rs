// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use log::{info};
use sha2::{Sha256, Digest};
use chrono::prelude::*;
use uuid::{Uuid};
use cmds::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;
use rand::{Rng, thread_rng};

#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    YCH,
    Commission,
    Request,
    Raffle,
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    pub name: String,
    pub contact: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(skip)]
    pub debug: Option<bool>,
}

impl Default for Category {
    fn default() -> Self { Category::Unknown }
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            name: "".to_string(),
            contact: "".to_string(),
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

    pub fn payment<S: Into<String>>(mut self, pay: S)
        -> Customer where S: Into<String> {
        self.payment = Some(pay.into());
        self
    }
}

impl Art {

    pub fn new() -> Art {
        Art {
            id: Uuid::new_v4().to_string(),
            date: Local::now(),
            version: "0.1".to_string(),
            name: "".to_string(),
            category: Some(Category::default()),
            customer: None,
            ticket: None,
            reference: None,
            slot: None,
            price: None,
            description: None,
            debug: None
        }
    }

    pub fn raffle<S: Into<String>>(mut self, ticket: S, slot: S) -> Art where S: Into<String> {
        let choosen_ticket = ticket.into();
        let choosen_slot = slot.into();
        let lt = Local::now();
        let tf = format!("{}{}{}{}{}", lt.year(), lt.month(), lt.day(), lt.hour(), lt.minute());
        let hasher = Sha256::digest(format!("artm:{}{}{}{}",
                                            tf, self.name, choosen_ticket, choosen_slot).as_bytes());

        self.ticket = Some(choosen_ticket);
        self.slot = Some(choosen_slot);
        self.id = format!("{:x}", hasher);

        self
    }

    pub fn secure_id(mut self) -> Art {
        let lt = Local::now();
        let tf = format!("{}{}{}{}", lt.month(), lt.day(), lt.hour(), lt.minute());
        let hasher = Sha256::digest(format!("artm:{}{}",
                                            tf, self.name.to_string()).as_bytes());

        self.id = format!("{:x}", hasher);

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

    pub fn reference<S: Into<String>>(mut self, reference: S)
                                        -> Art where S: Into<String> {
        self.reference = Some(reference.into());
        self
    }

    pub fn category(mut self, cat: Category) -> Art {
        match cat {
            Category::YCH => self.category = Some(Category::YCH),
            Category::Commission => self.category = Some(Category::Commission),
            Category::Request => self.category = Some(Category::Request),
            Category::Raffle => self.category = Some(Category::Raffle),
            _ => self.category = Some(Category::default()),
        }
        self
    }

    pub fn debug(mut self, debug: bool) -> Art {
        self.debug = Some(debug);

        self
    }

    pub fn write_file(&self) -> Result<()> {
        let json_string = serde_json::to_string_pretty(self)?;
        let is_debug = self.debug.unwrap();
        let cat = &self.category;
        let mut ticket = "".to_string();
        let mut slot = "".to_string();
        let name = self.name.to_owned();

        if is_debug == true {
            match cat {
                Some(Category::YCH) => {
                    slot = self.slot.to_owned().unwrap();
                    info!("YCH: {}, slot {}", name, slot);
                }
                Some(Category::Raffle) => {
                    ticket = self.ticket.to_owned().unwrap();
                    info!("Raffle: {}, ticket {}, slot {}", name, ticket, slot);
                }
                Some(Category::Request) => {
                    info!("Request: {}", name);
                }
                Some(Category::Commission) => {
                    info!("Commission: {}", name);
                }
                _ => {}
            }

            println!("{}", json_string);
        }
        else
        {
            let lt = Local::now();
            let tf = format!("{}{}{}", lt.year(), lt.month(), lt.day());
            let mut file_name = String::new();
            let file_log = "File:";

            match cat {
                Some(Category::YCH) => {
                    slot = self.slot.to_owned().unwrap();
                    file_name = format!("{} - {} - {}.arty",
                                        tf, name, slot);
                    info!("YCH: {}, slot {}", name, slot);
                    info!("{} \"{}\"", file_log, file_name);
                }
                Some(Category::Raffle) => {
                    slot = self.slot.to_owned().unwrap();
                    ticket = self.ticket.to_owned().unwrap();

                    file_name = format!("{} - {} - {} {}.arty",
                                        tf, name, ticket, slot);
                    info!("Raffle: {}, ticket {}, slot {}", name, ticket, slot);
                    info!("{} \"{}\"", file_log, file_name);
                }
                Some(Category::Request) => {
                    file_name = format!("{} - {}.artr", tf, self.name);
                    info!("Request: {}", name);
                    info!("{} \"{}\"", file_log, file_name);
                }
                Some(Category::Commission) => {
                    file_name = format!("{} - {}.artc", tf, name);
                    info!("Commission: {}", name);
                    info!("{} \"{}\"", file_log, file_name);
                }
                _ => {}
            }

            let mut file = OpenOptions::new().write(true)
                .create_new(true)
                .open(file_name)
                .expect("Could not open file.");

            if let Err(err) = writeln!(file, "{}", format!("{}", json_string)) {
                let err_msg = format!("{}: {}", ERROR_MSG, err);
                println!("{}", err_msg);
            }
        }

        Ok(())
    }
}

pub fn ych<S: Into<String>>(name: S, price: S, slot: S,
                            reference: S, cust_name: S,  pay: S,
                            cont: S, debug: bool) where S: Into<String> {
    if let Err(err) = Art::new()
        .price(price)
        .name(name)
        .category(Category::YCH)
        .slot(slot)
        .reference(reference)
        .customer(Customer::new()
            .name(cust_name)
            .payment(pay)
            .contact(cont))
        .secure_id()
        .debug(debug)
        .write_file() {
        let err_msg = format!("{}: {}", ERROR_MSG, err);
        println!("{}", err_msg);
        process::exit(EXIT_CODE);
    }
}

pub fn comm<S: Into<String>>(name: S, price: S, desc: S,
                             cust_name: S, pay: S,
                             cont: S, debug: bool) where S: Into<String> {
    if let Err(err) = Art::new()
        .price(price)
        .name(name)
        .category(Category::Commission)
        .customer(Customer::new()
            .name(cust_name)
            .payment(pay)
            .contact(cont))
        .description(desc)
        .secure_id()
        .debug(debug)
        .write_file() {
        let err_msg = format!("{}: {}", ERROR_MSG, err);
        println!("{}", err_msg);
        process::exit(EXIT_CODE);
    }
}

pub fn req<S: Into<String>>(name: S, desc: S, cust_name: S,
                            cont: S, debug: bool) where S: Into<String> {
    if let Err(err) = Art::new()
        .name(name)
        .category(Category::Request)
        .customer(Customer::new()
            .name(cust_name)
            .contact(cont))
        .description(desc)
        .secure_id()
        .debug(debug)
        .write_file() {
        let err_msg = format!("{}: {}", ERROR_MSG, err);
        println!("{}", err_msg);
        process::exit(EXIT_CODE);
    }
}

pub fn raffle<S: Into<String>>(name: S, tickets: i32, slots: i32, debug: bool) where S: Into<String> {
    let choose_ticket = format!("{}", thread_rng().gen_range(1, tickets));
    let choose_slot = format!("{}", thread_rng().gen_range(1, slots));

    if let Err(err) = Art::new()
        .name(name)
        .category(Category::Raffle)
        .raffle(choose_ticket, choose_slot)
        .debug(debug)
        .write_file() {
        let err_msg = format!("{}: {}", ERROR_MSG, err);
        println!("{}", err_msg);
        process::exit(EXIT_CODE);
    }
}