// Copyright (c) Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

use cda_sovd::dynamic_router::BaseUriPath;

use crate::config::configfile::Configuration;

pub trait VehicleConfigFromConfiguration {
    fn from_config(config: &Configuration) -> Self;
}

impl VehicleConfigFromConfiguration for cda_sovd::VehicleConfig {
    fn from_config(config: &Configuration) -> Self {
        Self {
            flash_files_path: config.flash_files_path.clone(),
            functional_group_config: config.functional_description.clone(),
            components_config: config.components.clone(),
            base_uri_path: BaseUriPath::new(format!(
                "/{}/{}",
                config.server.exve_manufacturer_specific, config.server.sovd_version_identifier
            )),
        }
    }
}
