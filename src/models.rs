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
use std::process::Command;

use inline_python::pyo3::prelude::*;
use inline_python::python;
use rayon::prelude::*;
use serde::Deserialize;

use crate::config;

pub struct Universe<'a> {
    config: &'a config::OneDSimulation,
    pub ex: Vec<f64>,
    pub hy: Vec<f64>,
    oscilloscopes: Vec<Oscilloscope<'a>>,
    signals: Vec<Signal<'a>>,
}

impl<'a> Universe<'a> {
    pub fn in_the_beginning(
        config: &'a config::OneDSimulation,
    ) -> Result<Universe<'a>, Box<dyn error::Error>> {
        let ex = (0..config.size).map(|_| 0.0).collect::<Vec<f64>>();
        let hy = (0..config.size).map(|_| 0.0).collect::<Vec<f64>>();

        let oscilloscopes = config
            .oscilloscopes
            .iter()
            .map(|c| Oscilloscope::new(c))
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

    pub fn let_there_be_light(&mut self) {
        rayon::scope(|thread_scope| {
            let ex = &mut self.ex;
            let hy = &mut self.hy;

            for t in 0..self.config.time {
                ex.par_iter_mut().enumerate().for_each(|(i, value)| {
                    if i != 0 {
                        *value += 0.5 * (hy[i - 1] - hy[i]);
                    }
                });

                for signal in &self.signals {
                    if let Some(value) = signal.bson.ex.get(t as usize) {
                        ex[signal.config.location] += value;
                    }
                }

                hy.par_iter_mut().enumerate().for_each(|(i, value)| {
                    if i != ex.len() - 1 {
                        *value += 0.5 * (ex[i] - ex[i + 1]);
                    }
                });

                for oscilloscope in self.oscilloscopes.iter_mut() {
                    oscilloscope.snapshot(thread_scope, t, ex, hy);
                }
            }
            for oscilloscope in self.oscilloscopes.iter_mut() {
                oscilloscope.flush(thread_scope);
            }
        });

        for oscilloscope in self.oscilloscopes.iter_mut() {
            oscilloscope.close();
        }
    }
}

pub struct Oscilloscope<'a> {
    config: &'a config::Oscilloscope,
    snapshots: Vec<Snapshot>,
}

impl<'b> Oscilloscope<'b> {
    pub fn new(config: &config::Oscilloscope) -> Oscilloscope {
        Oscilloscope {
            config,
            snapshots: vec![],
        }
    }

    pub fn close(&self) {
        match self.config {
            config::Oscilloscope::Movie(movie) => {
                let args = format!(
                    "-r {framerate} -f image2 -i t%04d.png -vcodec libx264 -crf 25 \
                    -pix_fmt yuv420p {path}",
                    framerate = movie.framerate,
                    path = movie.path
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

    pub fn flush<'a>(&mut self, thread_scope: &rayon::Scope<'a>) {
        match self.config {
            config::Oscilloscope::Movie(movie) => {
                let graph_period = movie.graph_period;
                let range = movie.range;
                let resolution = movie.resolution;
                let snapshots = self.snapshots.clone();
                thread_scope.spawn(move |_| {
                    python! {
                        import multiprocessing

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
                                pyplot.savefig(f"t{int(t/period):04}.png", dpi=my_dpi)
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

    pub fn snapshot<'a>(
        &mut self,
        thread_scope: &rayon::Scope<'a>,
        timestamp: u64,
        ex: &[f64],
        hy: &[f64],
    ) {
        match self.config {
            config::Oscilloscope::Movie(movie) => {
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalBSON {
    pub ex: Vec<f64>,
    _version: i64,
}

pub struct Signal<'a> {
    pub config: &'a config::Signal,
    pub bson: SignalBSON,
}

impl<'b> Signal<'b> {
    pub fn new(config: &config::Signal) -> Result<Signal, Box<dyn error::Error>> {
        let f = File::open(&config.path)?;
        let mut reader = BufReader::new(f);
        let bson = bson::decode_document(&mut reader)?;
        let bson: SignalBSON = bson::from_bson(bson::Bson::Document(bson))?;

        Ok(Signal { bson, config })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Snapshot {
    #[pyo3(get)]
    timestamp: u64,
    #[pyo3(get)]
    ex: Vec<f64>,
    #[pyo3(get)]
    hy: Vec<f64>,
}

impl ToPyObject for Snapshot {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyRefMut::new(
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
