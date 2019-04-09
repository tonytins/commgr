use clap::{crate_authors, crate_description, crate_version, AppSettings, load_yaml, App, ArgMatches};
use simplelog::*;
use std::fs::File;

// Argument names
// =======================
pub const RAFFLE_CMD: &str = "raffle";

pub const DEBUG_FLAG: &str = "debug";
pub const DATABASE_FLAG: &str = "database";

pub const CAT_OPT: &str = "category";
pub const CUST_NAME_OPT: &str = "customer";
pub const TICKETS_OPT: &str = "tickets";
pub const SLOTS_OPT: &str = "slots";
pub const CONTACT_OPT: &str = "contact";
pub const NAME_OPT: &str = "name";
pub const PAYMENT_OPT: &str = "payment";
pub const PRICE_OPT: &str = "price";
pub const SLOT_OPT: &str = "slot";
pub const REF_OPT: &str = "reference";
pub const DESC_OPT: &str = "description";
// =======================

// Messages
// =======================
pub const ERROR_MSG: &str = "Application error";
pub const CMD_NOT_FOUND_MSG: &str = "Command not found.";
// =======================

pub const EXIT_CODE: i32 = 1;

pub fn cli_arg() -> ArgMatches<'static> {
    App::from_yaml(load_yaml!("artm.yml"))
        .setting(AppSettings::VersionlessSubcommands)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches()
}