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

//! This module defines the schema and utilities for handling 1-dimensional simulation
//! configuration files.

use serde::Deserialize;

// Serde requires the use of functions to define defaults on struct fields, so these functions
// define those defaults.
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

/// The Movie allows the user to request that a video be made of the E and H values for the entire
/// simulation space across all of time.
#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Movie {
    /// The framerate for the resulting movie, in Hz.
    #[serde(default = "default_framerate")]
    pub framerate: u16,
    /// How often to capture a frame for the video, in simulation time steps.
    #[serde(default = "default_graph_period")]
    pub graph_period: u16,
    /// How large of a magnitude to graph on the y-axis.
    #[serde(default = "default_range")]
    pub range: f32,
    /// Where to store the movie at the end of the simulation.
    pub path: String,
    /// What resolution to use for the movie, in pixels.
    #[serde(default = "default_resolution")]
    pub resolution: (u16, u16),
    /// How many snapshots to buffer in memory before starting a child process to render them into
    /// movie frames.
    #[serde(default = "default_snapshot_buffer_len")]
    pub snapshot_buffer_len: u16,
}

/// An Oscilloscope is a tool for the user to request for simulation data to be captured.
#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Oscilloscope {
    /// Record a movie of the simulation.
    Movie(Movie),
}

/// Define a signal to place into the simulation space.
#[derive(Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Signal {
    /// Where in the simulation space to place the signal.
    pub location: usize,
    /// A path to a BSON file on disk that contains the signal values.
    pub path: String,
}

/// Define a configuration for a simulation.
#[derive(PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Simulation {
    /// A list of oscilloscopes to measure data in the simulation.
    pub oscilloscopes: Vec<Oscilloscope>,
    /// A list of signals to place into the simulation space.
    pub signals: Vec<Signal>,
    /// How many units large the universe is.
    pub size: u64,
    /// How long the universe will last until it all comes to an end.
    pub time: u64,
}
