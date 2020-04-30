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

//! # Simulation parameter configuration
//!
//! REMS expects you to define a simulation by providing it with a YAML file describing all the
//! parameters. Here is an example config showing all available options, with commented default
//! options:
//!
//! ```
//! ---
//! ## How many dimensions we have in space. Right now, only 1 is supported.
//! dimensions: 1
//! ## Define a list of signals
//! signals:
//!   ## This signal is at location 600 in space
//!   - location: 600
//!     ## Read this path to get the signal. See below for a description of this file.
//!     path: examples/1d_signal.bson
//! ## Define how large the universe should be, in cells
//! size: 1920
//! ## Define how many time steps the simulation should run for
//! time: 32768
//! ## This is a list of oscilloscopes, or outputs for your simulation. Right now, only the movie
//! ## type is supported, but one could imagine other types added in the future.
//! oscilloscopes:
//!   - type: movie
//!     # How large of a magnitude to use for the y-axis on the graphs
//!     ## range: 1.0
//!     ## The path to write the movie to
//!     path: examples/1d_simulation.mp4
//!     ## The framerate of the resulting movie, in Hz
//!     ## framerate: 60
//!     ## How often to generate a graph in simulation time steps
//!     ## graph_period: 16
//!     ## The resolution you desire for the resulting video, expressed as an array of two integers
//!     ## resolution:
//!     ##   - 1920
//!     ##   - 1080
//!     ## How many snapshots to buffer in memory before handing them off to a Python subprocess to
//!     ## generate graphs out of them.
//!     ## snapshot_buffer_len: 47
//! ```
//!
//! # The signal BSON file
//!
//! The signals should be defined in a BSON file, with three fields: ```ex```, ```_version```, and
//! ```dimensions```. ```_version``` should be set to 0, ```dimensions``` should be set to 1, and
//! ```ex``` should be an array of floating point numbers that express which value the signal
//! should have for each time step in the simulation. Here is a YAML example that corresponds in
//! spirit to that schema:
//!
//! ```
//! ---
//! _version: 0
//! dimensions: 1
//! ex:
//!   - 0.0
//!   - 0.1
//!   - 0.2
//!   - 0.3
//! ```

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
