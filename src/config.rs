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

use std::error;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

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
    let f = File::open(&config_file_path)?;
    let reader = BufReader::new(f);

    let config: Simulation = serde_yaml::from_reader(reader)?;
    Ok(config)
}

fn default_framerate() -> u16 {
    60
}
fn default_graph_period() -> u16 {
    16
}
fn default_range() -> f32 {
    1.0
}
fn default_resolution() -> (u16, u16) {
    (1920, 1080)
}
fn default_snapshot_buffer_len() -> u16 {
    47
}

#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Movie {
    #[serde(default = "default_framerate")]
    pub framerate: u16,
    #[serde(default = "default_graph_period")]
    pub graph_period: u16,
    #[serde(default = "default_range")]
    pub range: f32,
    pub path: String,
    #[serde(default = "default_resolution")]
    pub resolution: (u16, u16),
    #[serde(default = "default_snapshot_buffer_len")]
    pub snapshot_buffer_len: u16,
}

#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Oscilloscope {
    Movie(Movie),
}

#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Signal {
    pub location: usize,
    pub path: String,
}

#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct OneDSimulation {
    pub oscilloscopes: Vec<Oscilloscope>,
    pub signals: Vec<Signal>,
    pub size: u64,
    pub time: u64,
}

#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "dimensions")]
pub enum Simulation {
    #[serde(rename(deserialize = "1"))]
    OneDimensional(OneDSimulation),
}
