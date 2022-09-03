/*
 * Copyright © 2020 Randy Barlow
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! This module defines the schema and utilities for handling the configuration file.

use std::error;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

// This allows us to namespace 1D configuration models.
pub mod d1;

/// Return the user's config as a Simulation.
///
/// # Arguments
///
/// * `config_file_path` - A filesystem path to a YAML file that should be read.
///
/// # Returns
///
/// Returns a Simulation representing the parsed config, or an Error.
pub fn read_config(config_file_path: &str) -> Result<Simulation, Box<dyn error::Error>> {
    let f = File::open(config_file_path)?;
    let reader = BufReader::new(f);

    let config: Simulation = serde_yaml::from_reader(reader)?;
    Ok(config)
}

/// Define a configuration for a simulation.
#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "dimensions")]
pub enum Simulation {
    /// Define a configuration for a 1D simulation.
    #[serde(rename(deserialize = "1"))]
    OneDimensional(d1::Simulation),
}
