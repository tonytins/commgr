// Copyright (c) Tony Bark and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use crate::options::Order;
use chrono::{Datelike, Local};
use directories::UserDirs;
use std::io::Write;
use std::path::Path;
use std::{
    fs,
    fs::{File, OpenOptions},
};

pub fn simple_date() -> String {
    let dt_local = Local::now();
    format!(
        "{}/{}/{}",
        dt_local.month(),
        dt_local.day(),
        dt_local.year()
    )
}

fn content_manager<S: Into<String>>(file: S) -> (File, String) {
    // Create a new time card, if it doesn't exist
    let file_name = &docs_dir(file.into(), true);

    // Append header to order file
    let manger = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .expect("Error writing to file.");

    let contents = fs::read_to_string(file_name).expect("There was an error opening the file");

    (manger, contents)
}

pub fn docs_dir<S: Into<String>>(file: S, create_file: bool) -> String {
    let file_name = &file.into();
    let mut doc_file = String::new();
    let cdb_dir = "cdb";

    if let Some(user_dirs) = UserDirs::new() {
        let docs_dir = user_dirs
            .document_dir()
            .expect("There was an error detecting documents path.");
        let artm_path = format!("{}\\{}", docs_dir.display(), cdb_dir);

        if !Path::new(&artm_path).exists() {
            fs::create_dir(&artm_path).expect("There was an error creating the directory.");
        }

        doc_file = format!("{}\\{}\\{}", docs_dir.display(), cdb_dir, file_name);

        if !Path::new(&doc_file).exists() && create_file == true {
            File::create(&doc_file).expect("There was an error creating the file");
        }
    }

    doc_file
}

pub fn personal_manger<S: Into<String>>(file: S) -> File {
    // Create a new time card, if it doesn't exist
    let file_name = &docs_dir(file.into(), true);

    let (mut manger, contents) = content_manager(file_name);

    if contents.is_empty() {
        // Except a reference even if one is never provided
        let header = format!("Date,Name,Description,Reference");

        if let Err(err) = writeln!(manger, "{}", header) {
            eprintln!("Couldn't write to file: {}", err);
        }
    }

    manger
}

pub fn order_manager<S: Into<String>>(file: S, order: &Order) -> File {
    // Create a new time card, if it doesn't exist
    let file_name = &docs_dir(file.into(), true);

    let (mut manger, contents) = content_manager(file_name);

    if contents.is_empty() {
        let header = if order.ych.is_none() {
            format!("Date,Client,Fee,Payment,Description")
        } else {
            format!("Date,Client,Reference,Fee,Payment,YCH,Slot")
        };

        if let Err(err) = writeln!(manger, "{}", header) {
            eprintln!("Couldn't write to file: {}", err);
        }
    }

    manger
}
