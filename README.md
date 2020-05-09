```rems``` (Rust ElectroMagnetic Simulator) is a
[Finite Difference Time Domain](https://en.wikipedia.org/wiki/Finite-difference_time-domain_method)
(FDTD) simulator written in Rust.

It is capable of generating animated graphs of your simulations, like this:

![1D example simulation](https://user-images.githubusercontent.com/354506/80891137-14c73400-8c90-11ea-8ffb-af5c53f03a3a.gif)


# Install

```rems``` is [available on crates.io](https://crates.io/crates/rems). You can install rems by first
installing ```ffmpeg```, then [installing Rust](https://www.rust-lang.org/tools/install), and then
using Rust's ```rustup``` tool to install Rust nightly, and then you can use ```cargo``` to install
rems:

```
$ rustup install nightly
$ rustup override set nightly
# Be sure to read the output of this command and adjust your PATH as instructed.
$ cargo install rems
```


# Quick start

To get started with a quick simulation, have a look at the
[examples/](https://github.com/bowlofeggs/rems/blob/master/examples) folder. You will find two files
in there. ```1d_simulation.yml``` file is a simulation parameter file, and ```1d_signal.py``` is a
Python script that generates a signal in the format that rems expects. Install Python's bson
library, run the signal generator file, and then run the simulation:

```
$ pip install bson
$ python3 examples/1d_signal.py
$ rems examples/1d_simulation.yml
```

This will generate a video file on your system called ```simulation.mp4``` that will show you the
signal propagating in space. Good job!


# Simulation parameter configuration

REMS expects you to define a simulation by providing it with a YAML file describing all the
parameters. Here is an example config showing all available options, with commented default
options:

```
---
## How many dimensions we have in space. Right now, only 1 is supported.
dimensions: 1
## Define a list of signals
signals:
  ## This signal is at location 600 in space
  - location: 600
    ## Read this path to get the signal. See below for a description of this file.
    path: examples/1d_signal.bson
## Define how large the universe should be, in cells
size: 1920
## Define how many time steps the simulation should run for
time: 32768
## This is a list of oscilloscopes, or outputs for your simulation. Right now, only the movie
## type is supported, but one could imagine other types added in the future.
oscilloscopes:
  - type: movie
    # How large of a magnitude to use for the y-axis on the graphs
    ## range: 1.0
    ## The path to write the movie to
    path: examples/1d_simulation.mp4
    ## The framerate of the resulting movie, in Hz
    ## framerate: 60
    ## How often to generate a graph in simulation time steps
    ## graph_period: 16
    ## The resolution you desire for the resulting video, expressed as an array of two integers
    ## resolution:
    ##   - 1920
    ##   - 1080
    ## How many snapshots to buffer in memory before handing them off to a Python subprocess to
    ## generate graphs out of them.
    ## snapshot_buffer_len: 47
```

# The signal BSON file

The signals should be defined in a BSON file, with three fields: ```ex```, ```_version```, and
```dimensions```. ```_version``` should be set to 0, ```dimensions``` should be set to 1, and
```ex``` should be an array of floating point numbers that express which value the signal
should have for each time step in the simulation. Here is a YAML example that corresponds in
spirit to that schema:

```
---
_version: 0
dimensions: 1
ex:
  - 0.0
  - 0.1
  - 0.2
  - 0.3
```

# Links

* [Bugs](https://github.com/bowlofeggs/rems/issues)
* [Changelog](https://github.com/bowlofeggs/rems/blob/master/CHANGELOG.md)
* [Code](https://github.com/bowlofeggs/rems)
* [Documentation](https://docs.rs/rems)


# Contribute

If you would like to contribute to rems, send me a patch!

There are a few scripts in the ```devel/``` folder that are handy for development, if you have
podman on your system. To get started with them, run ```build.sh``` (root is not required) -
it will build a container on your system with podman that gathers and compiles rems's dependencies.
Once this container is built, you can run rems's tests with ```cargo.sh test```, and you can run
any other cargo command with ```cargo.sh``` (it accepts cargo's parameters).

Happy hacking!
