/*
 * Copyright © 2019-2020 Randy Barlow
 * Copyright © 2020 Jeremy Cline
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
use std::process::Command;

use inline_python::pyo3::prelude::*;
use inline_python::python;
use rayon::prelude::*;
use serde::Deserialize;
use tempfile::tempdir;

use crate::config;

/// The known universe.
pub struct Universe<'a> {
    /// Store a reference to our simulation configuration for easy access.
    config: &'a config::d1::Simulation,
    /// The electric field for the entire universe.
    pub ex: Vec<f64>,
    /// The magnetic field for the entire universe.
    pub hy: Vec<f64>,
    /// A list of oscilloscope objects that are capturing data about our universe.
    oscilloscopes: Vec<Oscilloscope<'a>>,
    /// A list of signals that are generating input into our universe.
    signals: Vec<Signal<'a>>,
}

impl<'a> Universe<'a> {
    /// Like new() for most Rust objects, but Biblical.
    ///
    /// # Arguments
    ///
    /// * `config` - A 1D configuration to define the Universe that will be created.
    ///
    /// # Returns
    ///
    /// A Universe, or an Error if one of the signals cannot be read.
    pub fn in_the_beginning(
        config: &'a config::d1::Simulation,
    ) -> Result<Universe<'a>, Box<dyn error::Error>> {
        let ex = (0..config.size).map(|_| 0.0).collect::<Vec<f64>>();
        let hy = (0..config.size).map(|_| 0.0).collect::<Vec<f64>>();

        let oscilloscopes = config
            .oscilloscopes
            .iter()
            .map(|c| Oscilloscope::new(c).unwrap())
            .collect::<Vec<_>>();
        let mut signals: Vec<Signal> = vec![];
        for c in &config.signals {
            signals.push(Signal::new(&c)?);
        }

        Ok(Universe {
            config,
            ex,
            hy,
            oscilloscopes,
            signals,
        })
    }

    /// Run the simulation.
    pub fn let_there_be_light(&mut self) {
        // We use a rayon scope so that we can wait for all graphs to finish being made before
        // exiting the function.
        rayon::scope(|thread_scope| {
            let ex = &mut self.ex;
            let hy = &mut self.hy;

            for t in 0..self.config.time {
                // Update the electric field based on the current values in the magnetic field.
                ex.par_iter_mut().enumerate().for_each(|(i, value)| {
                    if i != 0 {
                        *value += 0.5 * (hy[i - 1] - hy[i]);
                    }
                });

                // Inject the next value for each signal into the electric field.
                for signal in &self.signals {
                    if let Some(value) = signal.bson.ex.get(t as usize) {
                        ex[signal.config.location] += value;
                    }
                }

                // Update the magnetic field based on the current values in the electric field.
                hy.par_iter_mut().enumerate().for_each(|(i, value)| {
                    if i != ex.len() - 1 {
                        *value += 0.5 * (ex[i] - ex[i + 1]);
                    }
                });

                // Collect data about the current state of things with all of our oscilloscopes.
                for oscilloscope in self.oscilloscopes.iter_mut() {
                    oscilloscope.snapshot(thread_scope, t, ex, hy);
                }
            }
            // While we still have the rayon scope, let's use it to close our all of our graph
            // creation tasks.
            for oscilloscope in self.oscilloscopes.iter_mut() {
                oscilloscope.flush(thread_scope);
            }
        });

        // Clean up all oscilloscopes.
        for oscilloscope in self.oscilloscopes.iter_mut() {
            oscilloscope.close();
        }
    }
}

/// An Oscilloscope records data from the Universe.
pub struct Oscilloscope<'a> {
    /// The oscilloscope's configuration.
    config: &'a config::d1::Oscilloscope,
    /// A list of snapshots that the Oscilloscope has recorded.
    snapshots: Vec<Snapshot>,
    temp_dir: tempfile::TempDir,
}

impl<'b> Oscilloscope<'b> {
    /// Create an oscilloscope based on a configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for this oscilloscope.
    ///
    /// # Returns
    ///
    /// A new Oscilloscope. Congrats. Or an Error. Condolences.
    pub fn new(config: &config::d1::Oscilloscope) -> Result<Oscilloscope, Box<dyn error::Error>> {
        let temp_dir = tempdir()?;
        Ok(Oscilloscope {
            config,
            snapshots: vec![],
            temp_dir,
        })
    }

    /// Close the oscilloscope. A Movie scope will generate its movie at this step.
    pub fn close(&self) {
        match self.config {
            config::d1::Oscilloscope::Movie(movie) => {
                let args = format!(
                    "-r {framerate} -f image2 -i {temp_dir}/t%04d.png -vcodec libx264 -crf 25 \
                    -pix_fmt yuv420p {path}",
                    framerate = movie.framerate,
                    temp_dir = self
                        .temp_dir
                        .path()
                        .to_str()
                        .expect("Temporary directory path is invalid"),
                    path = movie.path,
                );
                let mut ffmpeg = Command::new("ffmpeg");
                ffmpeg.args(args.split(' '));
                let output = ffmpeg.output().expect("Failed to spawn ffmpeg");
                if !output.status.success() {
                    println!("{}", String::from_utf8(output.stdout).unwrap());
                    println!("{}", String::from_utf8(output.stderr).unwrap());
                }
            }
        }
    }

    /// Flush all the gathered snapshots to disk. For the movie scope, this will generate picture
    /// files that are some of the frames for the resulting movie.
    ///
    /// # Arguments
    ///
    /// * `thread_scope` - We need a Rayon thread scope so we can launch our Python rendering tasks
    ///   into it.
    pub fn flush<'a>(&mut self, thread_scope: &rayon::Scope<'a>) {
        match self.config {
            config::d1::Oscilloscope::Movie(movie) => {
                let graph_period = movie.graph_period;
                let range = movie.range;
                let resolution = movie.resolution;
                let snapshots = self.snapshots.clone();
                let temp_dir = self
                    .temp_dir
                    .path()
                    .to_str()
                    .expect("Temporary directory path is invalid")
                    .to_owned();
                thread_scope.spawn(move |_| {
                    python! {
                        import multiprocessing
                        import os

                        from matplotlib import pyplot


                        def graph(snapshots):
                            for snapshot in snapshots:
                                my_dpi = 96.0
                                period = 'graph_period
                                t = snapshot.timestamp
                                fig, ax = pyplot.subplots(figsize=('resolution[0]/my_dpi, 'resolution[1]/my_dpi))
                                ax.plot(range(0, len(snapshot.ex)), snapshot.ex, "b", label="electric field")
                                ax.plot(range(0, len(snapshot.hy)), snapshot.hy, "r", label="magnetic field")
                                pyplot.title(f"Time: {t}")
                                pyplot.xlabel("position")
                                pyplot.ylabel("magnitude")
                                pyplot.axis([0, len(snapshot.ex), -'range, 'range])
                                ax.legend()
                                pyplot.savefig(os.path.join('temp_dir, f"t{int(t/period):04}.png"), dpi=my_dpi)
                                pyplot.close(fig)


                        p = multiprocessing.Process(target=graph, args=('snapshots,))
                        p.start()
                        p.join()
                    }
                });
                self.snapshots.clear();
            }
        }
    }

    /// Take a snapshot of the Universe, as per this Oscilloscope's configuration.
    ///
    /// # Arguments
    ///
    /// * `thread_scope` - We use the Rayon thread scope so that we can call flush(), which needs
    ///   it.
    /// * `timestamp` - The time we are taking a snapshot of.
    /// * `ex` - A reference to the electric field we are snapshotting.
    /// * `hy` - A reference to the magnetic field we are snapshotting.
    pub fn snapshot<'a>(
        &mut self,
        thread_scope: &rayon::Scope<'a>,
        timestamp: u64,
        ex: &[f64],
        hy: &[f64],
    ) {
        match self.config {
            config::d1::Oscilloscope::Movie(movie) => {
                if timestamp % (movie.graph_period as u64) == 0 {
                    let snapshot = Snapshot {
                        timestamp,
                        ex: ex.to_owned(),
                        hy: hy.to_owned(),
                    };
                    self.snapshots.push(snapshot);
                    if self.snapshots.len() > movie.snapshot_buffer_len as usize {
                        self.flush(thread_scope);
                    }
                }
            }
        }
    }
}

/// This struct defines the schema for the BSON file that users encode the signals in.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalBson {
    /// The electric field values over time.
    pub ex: Vec<f64>,
    /// The version of the BSON file. This must be 0 for now.
    _version: i64,
}

/// This struct represents a signal in space, and is a wrapper around both the configuration for
/// the signal and the interpreted BSON data.
pub struct Signal<'a> {
    pub config: &'a config::d1::Signal,
    pub bson: SignalBson,
}

impl<'b> Signal<'b> {
    /// Initialize the signal, by opening and reading the referenced BSON data into memory.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration describing the Signal.
    ///
    /// # Returns
    ///
    /// A new signal, or an Error if the BSON file was not able to be read or was not valid.
    pub fn new(config: &config::d1::Signal) -> Result<Signal, Box<dyn error::Error>> {
        let f = File::open(&config.path)?;
        let mut reader = BufReader::new(f);
        let bson = bson::Document::from_reader(&mut reader)?;
        let bson: SignalBson = bson::from_bson(bson::Bson::Document(bson))?;

        Ok(Signal { config, bson })
    }
}

/// A recorded snapshot of the entire electric and magnetic field at a particular time.
#[pyclass]
#[derive(Clone)]
pub struct Snapshot {
    /// The time that this snapshot records.
    #[pyo3(get)]
    timestamp: u64,
    /// The electric field values for all of the Universe at this time.
    #[pyo3(get)]
    ex: Vec<f64>,
    /// The magnetic field values for all of the Universe at this time.
    #[pyo3(get)]
    hy: Vec<f64>,
}

impl ToPyObject for Snapshot {
    /// Convert the Snapshot into a Python object so that we can hand it into Python for graphing.
    ///
    /// # Arguments
    ///
    /// * `py` - The handle to the Python interpreter, needed to use the GIL.
    ///
    /// # Returns
    ///
    /// A PyObject representation of this Snapshot, which can be used in Python as a class with
    /// attributes that access the struct's fields.
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyCell::new(
            py,
            Snapshot {
                timestamp: self.timestamp,
                ex: self.ex.clone(),
                hy: self.hy.clone(),
            },
        )
        .expect("Unable to build Python Snapshot");
        dict.to_object(py)
    }
}
