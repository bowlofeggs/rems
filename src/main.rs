/*
 * Copyright © 2019-2020, 2022-2023 Randy Barlow
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

//! # REMS (The Rust ElectroMagnetic Simulator)
//!
//! REMS is a
//! [Finite Difference Time Domain](https://en.wikipedia.org/wiki/Finite-difference_time-domain_method)
//! (FDTD) simulator written in Rust.
//!
//! Check out the [README](https://github.com/bowlofeggs/rems/blob/master/README.md) for
//! installation and quick start guides.
//!
//! For a more detailed description of the configuration file format, see
//! [config](config/index.html).

#![feature(proc_macro_hygiene)]

use std::error;

use clap::Parser;

mod config;
mod models;

/// Define the CLI parameters
#[derive(Parser)]
struct Cli {
    /// A path to a simulation config
    config: String,
}

/// In the beginning, the world was public static void main…
fn main() {
    let args = Cli::parse();
    let simulation = config::read_config(&args.config);

    match simulation {
        Ok(simulation) => match simulation {
            config::Simulation::OneDimensional(config) => {
                let universe = models::Universe::in_the_beginning(&config);
                match universe {
                    Ok(mut universe) => {
                        universe.let_there_be_light();
                    }
                    Err(error) => {
                        handle_error(error);
                    }
                }
            }
        },
        Err(error) => {
            handle_error(error);
        }
    }
}

/// Print the error and exit, meanly.
///
/// # Arguments
///
/// * `error` - The error we sadly encountered.
fn handle_error(error: Box<dyn error::Error>) {
    println!("{error}");
    std::process::exit(1);
}
