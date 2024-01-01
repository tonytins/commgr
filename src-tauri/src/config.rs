// Copyright (c) Anthony Leland and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;

#[derive(Deserialize, Serialize, Clone)]
pub struct ManagerConfig {
    pub currency: String
}

impl Default for ManagerConfig {
    fn default() -> Self {
        ManagerConfig {
            currency: "USD".to_string()
        }
    }
}

pub fn get_config<S: Into<String>>(file: S) -> ManagerConfig {
    let file_name = &file.into();
    if Path::new(file_name).exists() {
        let contents = fs::read_to_string(file_name)
            .expect("Error reading file.");

        let config: ManagerConfig = toml::from_str(&contents)
            .expect("There was a problem with your configuration.");

        config
    } else {
        ManagerConfig::default()
    }
}