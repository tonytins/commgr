// Copyright (c) Tony Bark and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
// use clap::Clap;
use serde::Deserialize;

#[derive(Debug, Clone)]
// #[clap(author, about, version)]
pub struct Opts {
    // #[clap(short, long)]
    pub debug: Option<bool>,
    // #[clap(subcommand)]
    pub subcmds: SubCommands,
}

#[derive(Debug, Clone)]
pub enum SubCommands {
    // #[clap(author, about = "Allows for storing of commission and YCH information into CSV files.", version)]
    Order(Order),
    // #[clap(author, about = "Simpler variant of the order command for personal projects.", version)]
    Pers(Personal),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Personal {
    // #[clap(short, long)]
    pub name: String,
    // #[clap(short, long)]
    pub reference: Option<String>,
    // #[clap(short, long)]
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    // #[clap(short, long)]
    pub buyer: String,
    // #[clap(short, long)]
    pub currency: Option<String>, // Buyer's currency
    // #[clap(short, long)]
    pub fee: i32,
    // #[clap(short, long)]
    pub payment: String,
    // #[clap(short, long)]
    pub reference: Option<String>,
    // #[clap(short, long)]
    pub description: Option<String>,
    // #[clap(short, long)]
    pub ych: Option<String>, // YCH name
    // #[clap(short, long)]
    pub slot: Option<i32>, // YCH slot
}
