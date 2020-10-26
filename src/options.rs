// Copyright (c) Anthony Leland and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use clap::Clap;
use serde::Deserialize;

#[derive(Clap, Debug, Clone)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(short, long)]
    pub debug: Option<bool>,
    #[clap(short, long)]
    pub credits: Option<bool>,
    #[clap(subcommand)]
    pub order: Orders
}

#[derive(Clap, Debug, Clone)]
pub enum Orders {
    Order(Order)
}

#[derive(Clap, Debug, Deserialize, Clone)]
pub struct Order {
    #[clap(short, long)]
    pub client: String,
    #[clap(short, long)]
    pub fee: i32,
    #[clap(short, long)]
    pub payment: String,
    #[clap(short, long)]
    pub reference: Option<String>,
    #[clap(short, long)]
    pub description: Option<String>,
    #[clap(short, long)]
    pub ych: Option<String>, // YCH name
    #[clap(short, long)]
    pub slot: Option<i32>, // YCH slot
}