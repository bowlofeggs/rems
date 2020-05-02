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
